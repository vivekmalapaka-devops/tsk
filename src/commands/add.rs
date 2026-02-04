use crate::cli::parse_tags_from_text;
use crate::display::{print_error, print_todo_added, DisplayConfig};
use crate::store::Store;
use crate::time::parse_time;
use crate::todo::Todo;

pub fn run(
    text: Vec<String>,
    priority: Option<u8>,
    time: Option<String>,
    store: &mut Store,
    config: &DisplayConfig,
) {
    if text.is_empty() {
        print_error("Task text is required");
        return;
    }

    // Validate priority
    if let Some(p) = priority {
        if p < 1 || p > 3 {
            print_error("Priority must be 1, 2, or 3");
            return;
        }
    }

    // Parse text and extract tags
    let (task_text, tags) = parse_tags_from_text(&text);

    if task_text.is_empty() {
        print_error("Task text is required");
        return;
    }

    // Parse deadline
    let deadline = if let Some(t) = time {
        match parse_time(&t) {
            Some(dt) => Some(dt),
            None => {
                print_error(&format!("Could not parse time \"{}\"", t));
                return;
            }
        }
    } else {
        None
    };

    let todo = Todo::new(0, task_text)
        .with_priority(priority)
        .with_deadline(deadline)
        .with_tags(tags);

    let added = store.add(todo);
    print_todo_added(added, config);

    if let Err(e) = store.save() {
        print_error(&format!("Could not save: {}", e));
    }
}
