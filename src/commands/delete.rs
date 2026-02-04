use crate::display::{print_error, print_todo_deleted, DisplayConfig};
use crate::store::Store;

pub fn run(ids: Vec<u32>, store: &mut Store, config: &DisplayConfig) {
    if ids.is_empty() {
        print_error("At least one task ID is required");
        return;
    }

    let mut success = false;

    for id in ids {
        if let Some(todo) = store.remove(id) {
            print_todo_deleted(&todo, config);
            success = true;
        } else {
            print_error(&format!("Task #{} not found", id));
        }
    }

    if success {
        if let Err(e) = store.save() {
            print_error(&format!("Could not save: {}", e));
        }
    }
}
