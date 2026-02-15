mod output;
mod runner;
mod stats;
mod types;

use clap::Parser;
use owo_colors::OwoColorize;
use std::process;

#[derive(Parser, Debug)]
#[command(
    name = "clash",
    version,
    about = "⚔️  clash — benchmark comparator\n\nRun commands head-to-head and compare their performance.\nMeasures execution time AND peak memory usage.",
    long_about = None
)]
struct Cli {
    /// Commands to benchmark (at least 2)
    #[arg(required = true, num_args = 2..)]
    commands: Vec<String>,

    /// Number of benchmark runs per command
    #[arg(short, long, default_value_t = 5)]
    runs: usize,

    /// Number of warmup runs before benchmarking
    #[arg(short, long, default_value_t = 0)]
    warmup: usize,

    /// Export results to JSON file
    #[arg(short, long)]
    export: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    if cli.runs == 0 {
        eprintln!("{} --runs must be at least 1", "Error:".red().bold());
        process::exit(1);
    }

    println!();
    println!("  {}  clash — benchmark comparator", "⚔️".bold());
    println!();

    let mut all_stats = Vec::new();

    for (i, cmd) in cli.commands.iter().enumerate() {
        println!(
            "  [{}] Benchmarking: {}",
            (i + 1).to_string().cyan(),
            cmd.bold()
        );

        match runner::run_benchmark(cmd, cli.runs, cli.warmup) {
            Ok(results) => {
                let cmd_stats = stats::compute_stats(cmd, &results);

                if cmd_stats.failed_runs > 0 {
                    eprintln!(
                        "  {} {}/{} runs exited with non-zero status",
                        "Warning:".yellow().bold(),
                        cmd_stats.failed_runs,
                        cmd_stats.runs
                    );
                }

                all_stats.push(cmd_stats);
            }
            Err(e) => {
                eprintln!("  {} {}", "Error:".red().bold(), e);
                process::exit(1);
            }
        }
    }

    // Clear the benchmark output and print the report
    println!();
    output::print_report(&all_stats);

    // Export to JSON if requested
    if let Some(path) = &cli.export {
        match serde_json::to_string_pretty(&all_stats) {
            Ok(json) => match std::fs::write(path, &json) {
                Ok(_) => println!("  {} Results exported to {}", "✓".green(), path),
                Err(e) => eprintln!("  {} Failed to write {}: {}", "Error:".red().bold(), path, e),
            },
            Err(e) => eprintln!("  {} Failed to serialize results: {}", "Error:".red().bold(), e),
        }
    }
}
