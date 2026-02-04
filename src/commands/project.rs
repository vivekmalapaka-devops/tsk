use crate::display::DisplayConfig;
use crate::store::Store;
use crate::todo::Todo;
use colored::Colorize;
use std::collections::HashMap;

pub fn run(store: &Store, config: &DisplayConfig, project_name: &str) {
    let mut todos: Vec<&Todo> = store
        .open_todos()
        .filter(|t| t.in_project(project_name))
        .collect();

    let count = todos.len();

    // Print header
    let task_word = if count == 1 { "task" } else { "tasks" };
    let header = format!("Project: {} ({} {})", project_name, count, task_word);
    let separator = "─".repeat(38);

    if config.use_color {
        println!("  {}", header.bold());
        println!("  {}", separator.dimmed());
    } else {
        println!("  {}", header);
        println!("  {}", separator);
    }

    if todos.is_empty() {
        if config.use_color {
            println!("  {}", "No tasks in this project.".dimmed());
        } else {
            println!("  No tasks in this project.");
        }
        return;
    }

    // Sort by priority, then ID
    todos.sort_by(|a, b| {
        let pa = a.priority.unwrap_or(99);
        let pb = b.priority.unwrap_or(99);
        pa.cmp(&pb).then_with(|| a.id.cmp(&b.id))
    });

    for todo in todos {
        println!("{}", format_project_todo(todo, config));
    }
}

pub fn list_projects(store: &Store, config: &DisplayConfig) {
    let mut project_counts: HashMap<String, usize> = HashMap::new();

    for todo in store.open_todos() {
        if let Some(ref project) = todo.project {
            *project_counts.entry(project.clone()).or_insert(0) += 1;
        }
    }

    if project_counts.is_empty() {
        if config.use_color {
            println!("{}", "No projects found.".dimmed());
        } else {
            println!("No projects found.");
        }
        return;
    }

    // Sort alphabetically
    let mut projects: Vec<_> = project_counts.into_iter().collect();
    projects.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));

    for (project, count) in projects {
        let task_word = if count == 1 { "task" } else { "tasks" };
        if config.use_color {
            println!("  {} ({} {})", project.magenta(), count, task_word);
        } else {
            println!("  {} ({} {})", project, count, task_word);
        }
    }
}

fn format_project_todo(todo: &Todo, config: &DisplayConfig) -> String {
    let id = format!("{:>4}", todo.id);
    let priority = todo.priority_display();
    let text = truncate(&todo.text, 35);

    let deadline_str = if let Some(deadline) = todo.deadline {
        crate::time::format_deadline(deadline, todo.is_overdue())
    } else {
        "—".to_string()
    };

    let tags_str = if todo.tags.is_empty() {
        String::new()
    } else {
        todo.tags
            .iter()
            .map(|t| format!("+{}", t))
            .collect::<Vec<_>>()
            .join(" ")
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
        } else if todo
            .deadline
            .map(crate::time::is_due_today)
            .unwrap_or(false)
        {
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
        format!("{}...", truncated)
    }
}
