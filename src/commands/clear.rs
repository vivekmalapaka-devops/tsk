use crate::display::{print_error, DisplayConfig};
use crate::store::Store;
use colored::Colorize;

pub fn run(store: &mut Store, config: &DisplayConfig) {
    let count = store.clear_completed();

    if count == 0 {
        println!("No completed tasks to clear.");
        return;
    }

    let msg = format!("Cleared {} completed task{}.", count, if count == 1 { "" } else { "s" });

    if config.use_color {
        println!("{}", msg.green());
    } else {
        println!("{}", msg);
    }

    if let Err(e) = store.save_with_undo() {
        print_error(&format!("Could not save: {}", e));
    }
}
