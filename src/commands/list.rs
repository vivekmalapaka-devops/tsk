use crate::cli::SortBy;
use crate::display::{format_todo, print_empty_message, DisplayConfig};
use crate::store::Store;
use crate::time::{is_due_this_week, is_due_today};
use crate::todo::Todo;

pub enum Filter {
    Open,
    All,
    Today,
    Week,
    Overdue,
}

pub fn run(
    store: &Store,
    config: &DisplayConfig,
    sort_by: Option<SortBy>,
    filter: Filter,
    tag_filters: &[String],
) {
    let mut todos: Vec<&Todo> = match filter {
        Filter::Open => store.open_todos().collect(),
        Filter::All => store.todos.iter().collect(),
        Filter::Today => store
            .open_todos()
            .filter(|t| t.deadline.map(is_due_today).unwrap_or(false))
            .collect(),
        Filter::Week => store
            .open_todos()
            .filter(|t| t.deadline.map(is_due_this_week).unwrap_or(false))
            .collect(),
        Filter::Overdue => store.open_todos().filter(|t| t.is_overdue()).collect(),
    };

    // Apply tag filters
    if !tag_filters.is_empty() {
        todos.retain(|t| tag_filters.iter().all(|tag| t.has_tag(tag)));
    }

    if todos.is_empty() {
        print_empty_message();
        return;
    }

    // Sort
    let sort_by = sort_by.unwrap_or(SortBy::Priority);
    match sort_by {
        SortBy::Priority => {
            todos.sort_by(|a, b| {
                let pa = a.priority.unwrap_or(99);
                let pb = b.priority.unwrap_or(99);
                pa.cmp(&pb).then_with(|| a.id.cmp(&b.id))
            });
        }
        SortBy::Time => {
            todos.sort_by(|a, b| {
                match (&a.deadline, &b.deadline) {
                    (Some(da), Some(db)) => da.cmp(db),
                    (Some(_), None) => std::cmp::Ordering::Less,
                    (None, Some(_)) => std::cmp::Ordering::Greater,
                    (None, None) => a.id.cmp(&b.id),
                }
            });
        }
        SortBy::Created => {
            todos.sort_by(|a, b| a.created_at.cmp(&b.created_at));
        }
    }

    for todo in todos {
        println!("{}", format_todo(todo, config));
    }
}
