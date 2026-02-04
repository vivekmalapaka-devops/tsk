use crate::time::{format_completed_time, format_deadline};
use crate::todo::Todo;
use colored::Colorize;
use std::io::{self, IsTerminal};

pub struct DisplayConfig {
    pub use_color: bool,
}

impl DisplayConfig {
    pub fn new(force_color: Option<bool>) -> Self {
        let use_color = match force_color {
            Some(c) => c,
            None => io::stdout().is_terminal(),
        };
        Self { use_color }
    }
}

pub fn format_todo(todo: &Todo, config: &DisplayConfig) -> String {
    let id = format!("{:>3}", todo.id);
    let priority = todo.priority_display();
    let text = truncate(&todo.text, 35);

    let deadline_str = if todo.done {
        if let Some(completed) = todo.completed_at {
            format_completed_time(completed)
        } else {
            "done".to_string()
        }
    } else if let Some(deadline) = todo.deadline {
        format_deadline(deadline, todo.is_overdue())
    } else {
        "—".to_string()
    };

    let tags_str = if todo.tags.is_empty() {
        String::new()
    } else {
        todo.tags.iter().map(|t| format!("+{}", t)).collect::<Vec<_>>().join(" ")
    };

    let checkmark = if todo.done { "✓" } else { " " };

    if config.use_color {
        let id_colored = id.dimmed();

        let priority_colored = match todo.priority {
            Some(1) => "!!!".red().bold(),
            Some(2) => "!! ".yellow(),
            Some(3) => "!  ".blue(),
            _ => "   ".normal(),
        };

        let text_colored = if todo.done {
            text.dimmed().strikethrough()
        } else {
            text.normal()
        };

        let deadline_colored = if todo.done {
            deadline_str.green()
        } else if todo.is_overdue() {
            deadline_str.red().bold()
        } else if todo.deadline.map(|d| crate::time::is_due_today(d)).unwrap_or(false) {
            deadline_str.yellow()
        } else {
            deadline_str.normal()
        };

        let tags_colored = tags_str.cyan();
        let checkmark_colored = if todo.done { checkmark.green() } else { checkmark.normal() };

        format!(
            "{} {} {}  {:<35}  {:<18}  {}",
            checkmark_colored, id_colored, priority_colored, text_colored, deadline_colored, tags_colored
        )
    } else {
        format!(
            "{} {} {}  {:<35}  {:<18}  {}",
            checkmark, id, priority, text, deadline_str, tags_str
        )
    }
}

pub fn print_todo_added(todo: &Todo, config: &DisplayConfig) {
    let mut parts = vec![format!("Added #{}: {}", todo.id, todo.text)];

    if let Some(p) = todo.priority {
        let p_str = match p {
            1 => "[!!!]",
            2 => "[!!]",
            3 => "[!]",
            _ => "",
        };
        parts.push(p_str.to_string());
    }

    if let Some(deadline) = todo.deadline {
        parts.push(format_deadline(deadline, false));
    }

    if !todo.tags.is_empty() {
        let tags = todo.tags.iter().map(|t| format!("+{}", t)).collect::<Vec<_>>().join(" ");
        parts.push(tags);
    }

    let msg = parts.join(" ");
    if config.use_color {
        println!("{}", msg.green());
    } else {
        println!("{}", msg);
    }
}

pub fn print_todo_completed(todo: &Todo, config: &DisplayConfig) {
    let msg = format!("Completed #{}: {}", todo.id, todo.text);
    if config.use_color {
        println!("{}", msg.green());
    } else {
        println!("{}", msg);
    }
}

pub fn print_todo_deleted(todo: &Todo, config: &DisplayConfig) {
    let msg = format!("Deleted #{}: {}", todo.id, todo.text);
    if config.use_color {
        println!("{}", msg.yellow());
    } else {
        println!("{}", msg);
    }
}

pub fn print_todo_updated(todo: &Todo, config: &DisplayConfig) {
    let mut parts = vec![format!("Updated #{}: {}", todo.id, todo.text)];

    if let Some(p) = todo.priority {
        let p_str = match p {
            1 => "[!!!]",
            2 => "[!!]",
            3 => "[!]",
            _ => "",
        };
        parts.push(p_str.to_string());
    }

    if let Some(deadline) = todo.deadline {
        parts.push(format_deadline(deadline, false));
    }

    let msg = parts.join(" ");
    if config.use_color {
        println!("{}", msg.cyan());
    } else {
        println!("{}", msg);
    }
}

pub fn print_error(msg: &str) {
    eprintln!("{}: {}", "Error".red().bold(), msg);
}

pub fn print_empty_message() {
    println!("No open tasks. Use 'tsk add' to create one.");
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.chars().count() <= max_len {
        s.to_string()
    } else {
        let truncated: String = s.chars().take(max_len - 1).collect();
        format!("{}…", truncated)
    }
}
