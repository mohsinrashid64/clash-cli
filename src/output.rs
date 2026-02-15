use crate::stats;
use crate::types::CommandStats;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Attribute, Cell, Color, ContentArrangement, Table};
use owo_colors::OwoColorize;

/// Print the full benchmark comparison report.
pub fn print_report(all_stats: &[CommandStats]) {
    println!();
    println!(
        "  {}  clash — benchmark comparator",
        "⚔️".bold()
    );
    println!();

    // Print run summaries
    for s in all_stats {
        let status = if s.failed_runs == 0 {
            "✓".green().to_string()
        } else {
            format!("⚠ {} failed", s.failed_runs).yellow().to_string()
        };
        println!(
            "  {} {} ({} runs)",
            status,
            s.label.bold(),
            s.runs
        );
    }
    println!();

    // Time comparison table
    print_time_table(all_stats);
    println!();

    // Memory comparison table
    print_memory_table(all_stats);
    println!();

    // Overall summary
    print_summary(all_stats);
}

fn print_time_table(all_stats: &[CommandStats]) {
    let time_comp = stats::compare_time(all_stats);
    let winner_idx = time_comp.as_ref().map(|c| c.winner_index);

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic);

    // Header row
    let mut header = vec![Cell::new("⏱  Time")
        .add_attribute(Attribute::Bold)
        .fg(Color::Cyan)];
    for s in all_stats {
        header.push(Cell::new(&s.label).add_attribute(Attribute::Bold));
    }
    table.set_header(header);

    // Mean row
    let mut mean_row = vec![Cell::new("Mean")];
    for (i, s) in all_stats.iter().enumerate() {
        let cell = Cell::new(format_duration(s.time_mean));
        mean_row.push(if winner_idx == Some(i) {
            cell.fg(Color::Green).add_attribute(Attribute::Bold)
        } else {
            cell
        });
    }
    table.add_row(mean_row);

    // Min row
    let mut min_row = vec![Cell::new("Min")];
    for s in all_stats {
        min_row.push(Cell::new(format_duration(s.time_min)));
    }
    table.add_row(min_row);

    // Max row
    let mut max_row = vec![Cell::new("Max")];
    for s in all_stats {
        max_row.push(Cell::new(format_duration(s.time_max)));
    }
    table.add_row(max_row);

    // Std Dev row
    let mut std_row = vec![Cell::new("Std Dev")];
    for s in all_stats {
        std_row.push(Cell::new(format!("±{}", format_duration(s.time_std_dev))));
    }
    table.add_row(std_row);

    println!("{table}");

    // Bar chart
    print_bar_chart(
        all_stats,
        |s| s.time_mean.as_secs_f64(),
        |v| format_duration(std::time::Duration::from_secs_f64(v)),
        winner_idx,
    );

    // Comparison note
    if let Some(comp) = time_comp {
        if comp.ratio > 1.01 {
            println!(
                "  {} {} is {:.2}x faster",
                "→".cyan(),
                all_stats[comp.winner_index].label.green().bold(),
                comp.ratio
            );
        } else {
            println!("  {} Roughly the same speed", "→".cyan());
        }
    }
}

fn print_memory_table(all_stats: &[CommandStats]) {
    let mem_comp = stats::compare_memory(all_stats);
    let winner_idx = mem_comp.as_ref().map(|c| c.winner_index);

    // Check if we have any memory data
    if all_stats.iter().all(|s| s.peak_memory_bytes == 0) {
        println!(
            "  {} Memory data unavailable (processes too short-lived to measure)",
            "💾".dimmed()
        );
        return;
    }

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic);

    // Header row
    let mut header = vec![Cell::new("💾  Memory")
        .add_attribute(Attribute::Bold)
        .fg(Color::Magenta)];
    for s in all_stats {
        header.push(Cell::new(&s.label).add_attribute(Attribute::Bold));
    }
    table.set_header(header);

    // Peak RSS row
    let mut mem_row = vec![Cell::new("Peak RSS")];
    for (i, s) in all_stats.iter().enumerate() {
        let cell = Cell::new(format_bytes(s.peak_memory_bytes));
        mem_row.push(if winner_idx == Some(i) {
            cell.fg(Color::Green).add_attribute(Attribute::Bold)
        } else {
            cell
        });
    }
    table.add_row(mem_row);

    println!("{table}");

    // Bar chart
    print_bar_chart(
        all_stats,
        |s| s.peak_memory_bytes as f64,
        |v| format_bytes(v as u64),
        winner_idx,
    );

    // Comparison note
    if let Some(comp) = mem_comp {
        if comp.ratio > 1.01 {
            println!(
                "  {} {} uses {:.2}x less memory",
                "→".magenta(),
                all_stats[comp.winner_index].label.green().bold(),
                comp.ratio
            );
        } else {
            println!("  {} Roughly the same memory usage", "→".magenta());
        }
    }
}

fn print_bar_chart<F, G>(
    all_stats: &[CommandStats],
    value_fn: F,
    format_fn: G,
    winner_idx: Option<usize>,
) where
    F: Fn(&CommandStats) -> f64,
    G: Fn(f64) -> String,
{
    let max_bar_width = 30;
    let values: Vec<f64> = all_stats.iter().map(&value_fn).collect();
    let max_val = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    if max_val <= 0.0 {
        return;
    }

    // Find max label length for alignment
    let max_label_len = all_stats.iter().map(|s| s.label.len()).max().unwrap_or(0);

    for (i, s) in all_stats.iter().enumerate() {
        let val = value_fn(s);
        let bar_len = ((val / max_val) * max_bar_width as f64).round() as usize;
        let empty_len = max_bar_width - bar_len;

        let bar = "━".repeat(bar_len);
        let empty = "─".repeat(empty_len);
        let formatted_val = format_fn(val);

        let label_padded = format!("{:>width$}", s.label, width = max_label_len);

        if winner_idx == Some(i) {
            println!(
                "  {} {}{}  {}",
                label_padded.green(),
                bar.green(),
                empty.dimmed(),
                formatted_val.green()
            );
        } else {
            println!(
                "  {} {}{}  {}",
                label_padded,
                bar.red(),
                empty.dimmed(),
                formatted_val
            );
        }
    }
}

fn print_summary(all_stats: &[CommandStats]) {
    let time_comp = stats::compare_time(all_stats);
    let mem_comp = stats::compare_memory(all_stats);

    let mut parts = Vec::new();

    if let Some(tc) = time_comp {
        if tc.ratio > 1.01 {
            parts.push(format!(
                "{} wins on speed ({:.2}x)",
                all_stats[tc.winner_index].label, tc.ratio
            ));
        }
    }

    if let Some(mc) = mem_comp {
        if mc.ratio > 1.01 {
            parts.push(format!(
                "{} wins on memory ({:.2}x)",
                all_stats[mc.winner_index].label, mc.ratio
            ));
        }
    }

    if parts.is_empty() {
        println!("  {} Both commands perform similarly.", "Summary:".bold());
    } else {
        println!("  {} {}", "Summary:".bold(), parts.join(", "));
    }
    println!();
}

/// Format a Duration into a human-readable string.
fn format_duration(d: std::time::Duration) -> String {
    let secs = d.as_secs_f64();
    if secs >= 60.0 {
        let mins = (secs / 60.0).floor() as u64;
        let remaining = secs - (mins as f64 * 60.0);
        format!("{}m {:.3}s", mins, remaining)
    } else if secs >= 1.0 {
        format!("{:.3}s", secs)
    } else if secs >= 0.001 {
        format!("{:.1}ms", secs * 1000.0)
    } else {
        format!("{:.0}µs", secs * 1_000_000.0)
    }
}

/// Format bytes into a human-readable string.
fn format_bytes(bytes: u64) -> String {
    if bytes == 0 {
        return "N/A".to_string();
    }
    let kb = bytes as f64 / 1024.0;
    let mb = kb / 1024.0;
    let gb = mb / 1024.0;

    if gb >= 1.0 {
        format!("{:.1} GB", gb)
    } else if mb >= 1.0 {
        format!("{:.1} MB", mb)
    } else if kb >= 1.0 {
        format!("{:.1} KB", kb)
    } else {
        format!("{} B", bytes)
    }
}
