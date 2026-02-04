use crate::display::DisplayConfig;
use crate::store::Store;
use chrono::{Duration, Local};
use colored::Colorize;
use std::collections::HashMap;

pub fn run(store: &Store, config: &DisplayConfig) {
    let now = Local::now();
    let today_start = now.date_naive().and_hms_opt(0, 0, 0).unwrap();
    let week_start = now - Duration::days(7);

    let open_count = store.open_todos().count();
    let completed_count = store.completed_todos().count();

    let done_today = store
        .completed_todos()
        .filter(|t| {
            t.completed_at
                .map(|c| c.date_naive() >= today_start.date())
                .unwrap_or(false)
        })
        .count();

    let done_week = store
        .completed_todos()
        .filter(|t| t.completed_at.map(|c| c >= week_start).unwrap_or(false))
        .count();

    // Find oldest open task
    let oldest = store
        .open_todos()
        .min_by_key(|t| t.created_at)
        .map(|t| {
            let age = now - t.created_at;
            let age_str = if age.num_days() > 0 {
                format!("{} days", age.num_days())
            } else if age.num_hours() > 0 {
                format!("{} hours", age.num_hours())
            } else {
                "just now".to_string()
            };
            format!("{} (\"{}\")", age_str, truncate(&t.text, 20))
        });

    // Find most common tag
    let mut tag_counts: HashMap<&str, usize> = HashMap::new();
    for todo in store.open_todos() {
        for tag in &todo.tags {
            *tag_counts.entry(tag.as_str()).or_insert(0) += 1;
        }
    }
    let top_tag = tag_counts
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(tag, count)| format!("+{} ({} tasks)", tag, count));

    // Print stats
    if config.use_color {
        println!("{:<14} {}", "Open:".bold(), open_count);
        println!("{:<14} {}", "Completed:".bold(), completed_count);
        println!("{:<14} {}", "Done today:".bold(), done_today);
        println!("{:<14} {}", "Done week:".bold(), done_week);
        if let Some(o) = oldest {
            println!("{:<14} {}", "Oldest:".bold(), o);
        }
        if let Some(t) = top_tag {
            println!("{:<14} {}", "Top tag:".bold(), t.cyan());
        }
    } else {
        println!("{:<14} {}", "Open:", open_count);
        println!("{:<14} {}", "Completed:", completed_count);
        println!("{:<14} {}", "Done today:", done_today);
        println!("{:<14} {}", "Done week:", done_week);
        if let Some(o) = oldest {
            println!("{:<14} {}", "Oldest:", o);
        }
        if let Some(t) = top_tag {
            println!("{:<14} {}", "Top tag:", t);
        }
    }
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.chars().count() <= max_len {
        s.to_string()
    } else {
        let truncated: String = s.chars().take(max_len - 1).collect();
        format!("{}…", truncated)
    }
}
