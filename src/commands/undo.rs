use crate::display::{print_undo_success, DisplayConfig};
use crate::store::Store;

pub fn run(config: &DisplayConfig) {
    match Store::undo() {
        Ok(store) => {
            print_undo_success(store.todos.len(), config);
        }
        Err(_) => {
            println!("Nothing to undo.");
        }
    }
}
