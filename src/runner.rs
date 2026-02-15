use crate::types::RunResult;
use indicatif::{ProgressBar, ProgressStyle};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, System};

/// Run a single iteration of a command, measuring time and peak memory.
fn run_once(cmd: &str) -> Result<RunResult, String> {
    // Parse command into program + args (shell-style)
    let parts = shell_split(cmd)?;
    let (program, args) = parts
        .split_first()
        .ok_or_else(|| "Empty command".to_string())?;

    let mut child = Command::new(program)
        .args(args)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .map_err(|e| format!("Failed to start '{}': {}", cmd, e))?;

    let pid = child.id();
    let peak_memory = Arc::new(AtomicU64::new(0));
    let process_alive = Arc::new(std::sync::atomic::AtomicBool::new(true));

    // Spawn memory monitoring thread
    let monitor_handle = {
        let peak = peak_memory.clone();
        let alive = process_alive.clone();
        std::thread::spawn(move || {
            let mut sys = System::new();
            let refresh_kind = ProcessRefreshKind::nothing()
                .with_memory();

            while alive.load(Ordering::Relaxed) {
                sys.refresh_processes_specifics(
                    ProcessesToUpdate::Some(&[Pid::from_u32(pid)]),
                    true,
                    refresh_kind,
                );

                if let Some(proc) = sys.process(Pid::from_u32(pid)) {
                    peak.fetch_max(proc.memory(), Ordering::Relaxed);
                }

                std::thread::sleep(Duration::from_millis(30));
            }

            // One final check
            sys.refresh_processes_specifics(
                ProcessesToUpdate::Some(&[Pid::from_u32(pid)]),
                true,
                refresh_kind,
            );
            if let Some(proc) = sys.process(Pid::from_u32(pid)) {
                peak.fetch_max(proc.memory(), Ordering::Relaxed);
            }
        })
    };

    let start = Instant::now();
    let status = child.wait().map_err(|e| format!("Failed to wait for '{}': {}", cmd, e))?;
    let duration = start.elapsed();

    process_alive.store(false, Ordering::Relaxed);
    monitor_handle.join().ok();

    Ok(RunResult {
        duration,
        peak_memory_bytes: peak_memory.load(Ordering::Relaxed),
        exit_code: status.code(),
    })
}

/// Run a command multiple times with optional warmup, showing progress.
pub fn run_benchmark(
    cmd: &str,
    runs: usize,
    warmup: usize,
) -> Result<Vec<RunResult>, String> {
    // Warmup runs (not measured)
    if warmup > 0 {
        let warmup_pb = ProgressBar::new(warmup as u64);
        warmup_pb.set_style(
            ProgressStyle::with_template("    Warmup  {bar:20.dim} {pos}/{len}")
                .unwrap()
                .progress_chars("━━─"),
        );
        for _ in 0..warmup {
            run_once(cmd)?;
            warmup_pb.inc(1);
        }
        warmup_pb.finish_and_clear();
    }

    // Benchmark runs
    let pb = ProgressBar::new(runs as u64);
    pb.set_style(
        ProgressStyle::with_template("    Running {bar:20.cyan/dim} {pos}/{len} runs")
            .unwrap()
            .progress_chars("━━─"),
    );

    let mut results = Vec::with_capacity(runs);
    for _ in 0..runs {
        let result = run_once(cmd)?;
        results.push(result);
        pb.inc(1);
    }
    pb.finish_and_clear();

    Ok(results)
}

/// Simple shell-like argument splitting.
/// Handles double quotes and single quotes.
fn shell_split(cmd: &str) -> Result<Vec<String>, String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut in_single_quote = false;
    let mut in_double_quote = false;
    let mut chars = cmd.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '\'' if !in_double_quote => {
                in_single_quote = !in_single_quote;
            }
            '"' if !in_single_quote => {
                in_double_quote = !in_double_quote;
            }
            ' ' | '\t' if !in_single_quote && !in_double_quote => {
                if !current.is_empty() {
                    parts.push(current.clone());
                    current.clear();
                }
            }
            '\\' if in_double_quote || (!in_single_quote && !in_double_quote) => {
                if let Some(next) = chars.next() {
                    current.push(next);
                }
            }
            _ => {
                current.push(c);
            }
        }
    }

    if in_single_quote || in_double_quote {
        return Err("Unclosed quote in command".to_string());
    }

    if !current.is_empty() {
        parts.push(current);
    }

    if parts.is_empty() {
        return Err("Empty command".to_string());
    }

    Ok(parts)
}
