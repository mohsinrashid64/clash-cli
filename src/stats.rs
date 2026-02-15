use crate::types::{CommandStats, Comparison, RunResult};
use std::time::Duration;

/// Compute aggregated statistics from a set of run results.
pub fn compute_stats(command: &str, results: &[RunResult]) -> CommandStats {
    let durations: Vec<f64> = results.iter().map(|r| r.duration.as_secs_f64()).collect();
    let n = durations.len() as f64;

    let time_mean_f = durations.iter().sum::<f64>() / n;
    let time_min_f = durations.iter().cloned().fold(f64::INFINITY, f64::min);
    let time_max_f = durations.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    let time_std_dev_f = if durations.len() > 1 {
        let variance = durations
            .iter()
            .map(|d| (d - time_mean_f).powi(2))
            .sum::<f64>()
            / (n - 1.0);
        variance.sqrt()
    } else {
        0.0
    };

    let peak_memory = results.iter().map(|r| r.peak_memory_bytes).max().unwrap_or(0);
    let failed_runs = results
        .iter()
        .filter(|r| r.exit_code != Some(0))
        .count();

    // Create a short label from the command
    let label = make_label(command);

    CommandStats {
        command: command.to_string(),
        label,
        runs: results.len(),
        time_mean: Duration::from_secs_f64(time_mean_f),
        time_min: Duration::from_secs_f64(time_min_f),
        time_max: Duration::from_secs_f64(time_max_f),
        time_std_dev: Duration::from_secs_f64(time_std_dev_f),
        peak_memory_bytes: peak_memory,
        all_runs: results.to_vec(),
        failed_runs,
    }
}

/// Compare two stats on time — returns which is faster and by how much.
pub fn compare_time(stats: &[CommandStats]) -> Option<Comparison> {
    if stats.len() < 2 {
        return None;
    }
    let times: Vec<f64> = stats.iter().map(|s| s.time_mean.as_secs_f64()).collect();
    let (min_idx, min_val) = times
        .iter()
        .enumerate()
        .min_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .unwrap();
    let max_val = times
        .iter()
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max);

    Some(Comparison {
        winner_index: min_idx,
        ratio: max_val / min_val,
    })
}

/// Compare two stats on memory — returns which uses less and by how much.
pub fn compare_memory(stats: &[CommandStats]) -> Option<Comparison> {
    if stats.len() < 2 {
        return None;
    }
    let mems: Vec<u64> = stats.iter().map(|s| s.peak_memory_bytes).collect();

    // Skip comparison if any memory is 0 (unmeasurable)
    if mems.iter().any(|&m| m == 0) {
        return None;
    }

    let (min_idx, &min_val) = mems
        .iter()
        .enumerate()
        .min_by_key(|&(_, &v)| v)
        .unwrap();
    let &max_val = mems.iter().max().unwrap();

    Some(Comparison {
        winner_index: min_idx,
        ratio: max_val as f64 / min_val as f64,
    })
}

/// Create a short display label from a command string.
fn make_label(cmd: &str) -> String {
    let trimmed = cmd.trim();
    if trimmed.len() <= 30 {
        trimmed.to_string()
    } else {
        format!("{}...", &trimmed[..27])
    }
}
