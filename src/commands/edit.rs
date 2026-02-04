use crate::cli::{parse_project_from_text, parse_tag_modifications};
use crate::display::{print_error, print_todo_updated, DisplayConfig};
use crate::store::Store;
use crate::time::parse_time;

pub fn run(
    id: u32,
    text: Vec<String>,
    priority: Option<u8>,
    time: Option<String>,
    clear_time: bool,
    clear_priority: bool,
    clear_project: bool,
    store: &mut Store,
    config: &DisplayConfig,
) {
    // Validate priority
    if let Some(p) = priority {
        if p < 1 || p > 3 {
            print_error("Priority must be 1, 2, or 3");
            return;
        }
    }

    let todo = match store.get_mut(id) {
        Some(t) => t,
        None => {
            print_error(&format!("Task #{} not found", id));
            return;
        }
    };

    // Parse tag modifications from text
    let (add_tags, remove_tags) = parse_tag_modifications(&text);

    // Parse project from text
    let project = parse_project_from_text(&text);

    // Extract actual text (excluding tags and project)
    let new_text: String = text
        .iter()
        .filter(|p| !p.starts_with('+') && !p.starts_with('-') && !p.starts_with('@'))
        .cloned()
        .collect::<Vec<_>>()
        .join(" ");

    // Update text if provided
    if !new_text.is_empty() {
        todo.text = new_text;
    }

    // Update priority
    if clear_priority {
        todo.priority = None;
    } else if let Some(p) = priority {
        todo.priority = Some(p);
    }

    // Update deadline
    if clear_time {
        todo.deadline = None;
    } else if let Some(t) = time {
        match parse_time(&t) {
            Some(dt) => todo.deadline = Some(dt),
            None => {
                print_error(&format!("Could not parse time \"{}\"", t));
                return;
            }
        }
    }

    // Update tags
    for tag in add_tags {
        todo.add_tag(tag);
    }
    for tag in remove_tags {
        todo.remove_tag(&tag);
    }

    // Update project
    if clear_project {
        todo.project = None;
    } else if project.is_some() {
        todo.project = project;
    }

    print_todo_updated(todo, config);

    if let Err(e) = store.save() {
        print_error(&format!("Could not save: {}", e));
    }
}
