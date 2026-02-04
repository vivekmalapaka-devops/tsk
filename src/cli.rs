use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "tsk")]
#[command(author, version, about = "A fast, minimal terminal todo manager")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,

    /// Sort by: priority, time, created
    #[arg(long = "by", global = true)]
    pub sort_by: Option<SortBy>,

    /// Force color output
    #[arg(long, global = true)]
    pub color: bool,

    /// Disable color output
    #[arg(long, global = true)]
    pub no_color: bool,

    /// Filter by tag (can be repeated)
    #[arg(short = 'T', long = "tag", global = true)]
    pub tags: Vec<String>,

    /// Filter by project
    #[arg(short = 'P', long = "project", global = true)]
    pub project: Option<String>,
}

#[derive(Clone, Copy, clap::ValueEnum)]
pub enum SortBy {
    Priority,
    Time,
    Created,
}

#[derive(Subcommand)]
pub enum Command {
    /// Add a new task
    #[command(alias = "a")]
    Add {
        /// Task description
        text: Vec<String>,

        /// Priority (1=high, 2=medium, 3=low)
        #[arg(short, long)]
        p: Option<u8>,

        /// Deadline time
        #[arg(short, long)]
        t: Option<String>,
    },

    /// List all tasks (alias)
    Ls,

    /// Show all tasks including completed
    All,

    /// Mark task(s) as done
    #[command(alias = "d")]
    Done {
        /// Task ID(s) to complete
        ids: Vec<u32>,
    },

    /// Delete task(s)
    #[command(alias = "rm")]
    Delete {
        /// Task ID(s) to delete
        ids: Vec<u32>,
    },

    /// Edit a task
    #[command(alias = "e")]
    Edit {
        /// Task ID to edit
        id: u32,

        /// New task text
        text: Vec<String>,

        /// New priority (1=high, 2=medium, 3=low)
        #[arg(short, long)]
        p: Option<u8>,

        /// New deadline
        #[arg(short, long)]
        t: Option<String>,

        /// Clear deadline
        #[arg(long)]
        clear_time: bool,

        /// Clear priority
        #[arg(long)]
        clear_priority: bool,

        /// Clear project
        #[arg(long)]
        clear_project: bool,
    },

    /// Clear all completed tasks
    Clear,

    /// Show task statistics
    Stats,

    /// Show tasks due today
    Today,

    /// Show tasks due this week
    Week,

    /// Show overdue tasks
    Overdue,

    /// Show tasks in a project
    #[command(name = "project")]
    Project {
        /// Project name
        name: String,
    },

    /// List all projects
    #[command(name = "projects")]
    Projects,
}

impl Cli {
    pub fn get_color_mode(&self) -> Option<bool> {
        if self.color {
            Some(true)
        } else if self.no_color {
            Some(false)
        } else {
            None
        }
    }
}

pub fn parse_tags_from_text(parts: &[String]) -> (String, Vec<String>) {
    let mut text_parts = Vec::new();
    let mut tags = Vec::new();

    for part in parts {
        if part.starts_with('+') && part.len() > 1 {
            tags.push(part[1..].to_string());
        } else if part.starts_with('-') && part.len() > 1 {
            // Skip removal tags in add context
            continue;
        } else if part.starts_with('@') && part.len() > 1 {
            // Skip project markers - handled separately
            continue;
        } else {
            text_parts.push(part.clone());
        }
    }

    (text_parts.join(" "), tags)
}

pub fn parse_project_from_text(parts: &[String]) -> Option<String> {
    // Last @project wins if multiple specified
    let mut project = None;

    for part in parts {
        if part.starts_with('@') && part.len() > 1 {
            project = Some(part[1..].to_string());
        }
    }

    project
}

pub fn parse_tag_modifications(parts: &[String]) -> (Vec<String>, Vec<String>) {
    let mut add_tags = Vec::new();
    let mut remove_tags = Vec::new();

    for part in parts {
        if part.starts_with('+') && part.len() > 1 {
            add_tags.push(part[1..].to_string());
        } else if part.starts_with('-') && part.len() > 1 {
            remove_tags.push(part[1..].to_string());
        }
    }

    (add_tags, remove_tags)
}
