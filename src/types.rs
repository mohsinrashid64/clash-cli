use serde::Serialize;
use std::time::Duration;

/// Result of a single benchmark run
#[derive(Debug, Clone, Serialize)]
pub struct RunResult {
    pub duration: Duration,
    pub peak_memory_bytes: u64,
    pub exit_code: Option<i32>,
}

/// Aggregated statistics for all runs of a single command
#[derive(Debug, Clone, Serialize)]
pub struct CommandStats {
    pub command: String,
    pub label: String,
    pub runs: usize,
    pub time_mean: Duration,
    pub time_min: Duration,
    pub time_max: Duration,
    pub time_std_dev: Duration,
    pub peak_memory_bytes: u64,
    pub all_runs: Vec<RunResult>,
    pub failed_runs: usize,
}

/// Comparison between two commands for a specific metric
#[derive(Debug)]
pub struct Comparison {
    pub winner_index: usize,
    pub ratio: f64,
}
