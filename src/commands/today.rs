use crate::display::DisplayConfig;
use crate::store::Store;
use crate::time::{format_deadline, is_due_today};
use crate::todo::Todo;
use chrono::Local;
use colored::Colorize;

pub fn run(store: &Store, config: &DisplayConfig) {
    let now = Local::now();

    // Collect tasks into categories
    let mut overdue: Vec<&Todo> = Vec::new();
    let mut high_priority_today: Vec<&Todo> = Vec::new();
    let mut today_tasks: Vec<&Todo> = Vec::new();
    let mut high_priority_no_deadline: Vec<&Todo> = Vec::new();

    for todo in store.open_todos() {
        let is_high_priority = todo.priority == Some(1);
        let is_overdue = todo.is_overdue();
        let is_today = todo.deadline.map(is_due_today).unwrap_or(false);

        if is_overdue {
            overdue.push(todo);
        } else if is_today && is_high_priority {
            high_priority_today.push(todo);
        } else if is_today {
            today_tasks.push(todo);
        } else if is_high_priority && todo.deadline.is_none() {
            high_priority_no_deadline.push(todo);
        }
    }

    // Sort each section by priority, then by ID
    let sort_fn = |a: &&Todo, b: &&Todo| {
        let pa = a.priority.unwrap_or(99);
        let pb = b.priority.unwrap_or(99);
        pa.cmp(&pb).then_with(|| a.id.cmp(&b.id))
    };

    overdue.sort_by(sort_fn);
    high_priority_today.sort_by(sort_fn);
    today_tasks.sort_by(sort_fn);
    high_priority_no_deadline.sort_by(sort_fn);

    // Check if there's anything to show
    let total = overdue.len() + high_priority_today.len() + today_tasks.len() + high_priority_no_deadline.len();

    if total == 0 {
        if config.use_color {
            println!("{}", "No tasks for today. Enjoy your day!".dimmed());
        } else {
            println!("No tasks for today. Enjoy your day!");
        }
        return;
    }

    // Print header
    let date_str = now.format("%A, %B %-d").to_string();
    let header = format!("Today's Tasks - {}", date_str);
    let separator = "─".repeat(44);

    if config.use_color {
        println!("  {}", header.bold());
        println!("  {}", separator.dimmed());
    } else {
        println!("  {}", header);
        println!("  {}", separator);
    }

    // Print sections
    if !overdue.is_empty() {
        print_section("OVERDUE", &overdue, config, SectionStyle::Red);
    }

    if !high_priority_today.is_empty() {
        print_section("HIGH PRIORITY - TODAY", &high_priority_today, config, SectionStyle::Yellow);
    }

    if !today_tasks.is_empty() {
        print_section("TODAY", &today_tasks, config, SectionStyle::Normal);
    }

    if !high_priority_no_deadline.is_empty() {
        print_section("HIGH PRIORITY (no deadline)", &high_priority_no_deadline, config, SectionStyle::Cyan);
    }

    // Print summary
    let overdue_count = overdue.len();
    let high_priority_count = high_priority_today.len() + high_priority_no_deadline.len();

    let summary = format!(
        "{} task{} total | {} overdue | {} high priority",
        total,
        if total == 1 { "" } else { "s" },
        overdue_count,
        high_priority_count
    );

    if config.use_color {
        if overdue_count > 0 {
            println!("  {}", summary.red());
        } else {
            println!("  {}", summary.dimmed());
        }
    } else {
        println!("  {}", summary);
    }
}

enum SectionStyle {
    Red,
    Yellow,
    Normal,
    Cyan,
}

fn print_section(title: &str, todos: &[&Todo], config: &DisplayConfig, style: SectionStyle) {
    println!();

    let header = format!("{} ({})", title, todos.len());
    if config.use_color {
        match style {
            SectionStyle::Red => println!("  {}", header.red().bold()),
            SectionStyle::Yellow => println!("  {}", header.yellow().bold()),
            SectionStyle::Normal => println!("  {}", header.bold()),
            SectionStyle::Cyan => println!("  {}", header.cyan().bold()),
        }
    } else {
        println!("  {}", header);
    }

    for todo in todos {
        println!("{}", format_today_todo(todo, config));
    }
}

fn format_today_todo(todo: &Todo, config: &DisplayConfig) -> String {
    let id = format!("{:>4}", todo.id);
    let priority = todo.priority_display();
    let text = truncate(&todo.text, 35);

    let deadline_str = if let Some(deadline) = todo.deadline {
        format_deadline(deadline, todo.is_overdue())
    } else {
        "—".to_string()
    };

    let tags_str = if todo.tags.is_empty() {
        String::new()
    } else {
        todo.tags.iter().map(|t| format!("+{}", t)).collect::<Vec<_>>().join(" ")
    };

    if config.use_color {
        let id_colored = id.dimmed();

        let priority_colored = match todo.priority {
            Some(1) => "!!!".red().bold(),
            Some(2) => "!! ".yellow(),
            Some(3) => "!  ".blue(),
            _ => "   ".normal(),
        };

        let deadline_colored = if todo.is_overdue() {
            deadline_str.red().bold()
        } else if todo.deadline.map(is_due_today).unwrap_or(false) {
            deadline_str.yellow()
        } else {
            deadline_str.normal()
        };

        let tags_colored = tags_str.cyan();

        format!(
            "  {} {}  {:<35}  {:<18}  {}",
            id_colored, priority_colored, text, deadline_colored, tags_colored
        )
    } else {
        format!(
            "  {} {}  {:<35}  {:<18}  {}",
            id, priority, text, deadline_str, tags_str
        )
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
