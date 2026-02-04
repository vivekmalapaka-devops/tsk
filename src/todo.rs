use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: u32,
    pub text: String,
    pub done: bool,
    pub priority: Option<u8>,
    pub deadline: Option<DateTime<Local>>,
    pub tags: Vec<String>,
    pub project: Option<String>,
    pub created_at: DateTime<Local>,
    pub completed_at: Option<DateTime<Local>>,
}

impl Todo {
    pub fn new(id: u32, text: String) -> Self {
        Self {
            id,
            text,
            done: false,
            priority: None,
            deadline: None,
            tags: Vec::new(),
            project: None,
            created_at: Local::now(),
            completed_at: None,
        }
    }

    pub fn with_priority(mut self, priority: Option<u8>) -> Self {
        self.priority = priority;
        self
    }

    pub fn with_deadline(mut self, deadline: Option<DateTime<Local>>) -> Self {
        self.deadline = deadline;
        self
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    pub fn with_project(mut self, project: Option<String>) -> Self {
        self.project = project;
        self
    }

    pub fn in_project(&self, project: &str) -> bool {
        self.project
            .as_ref()
            .map(|p| p.eq_ignore_ascii_case(project))
            .unwrap_or(false)
    }

    pub fn mark_done(&mut self) {
        self.done = true;
        self.completed_at = Some(Local::now());
    }

    pub fn priority_display(&self) -> &'static str {
        match self.priority {
            Some(1) => "!!!",
            Some(2) => "!! ",
            Some(3) => "!  ",
            _ => "   ",
        }
    }

    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t.eq_ignore_ascii_case(tag))
    }

    pub fn add_tag(&mut self, tag: String) {
        if !self.has_tag(&tag) {
            self.tags.push(tag);
        }
    }

    pub fn remove_tag(&mut self, tag: &str) {
        self.tags.retain(|t| !t.eq_ignore_ascii_case(tag));
    }

    pub fn is_overdue(&self) -> bool {
        if let Some(deadline) = self.deadline {
            !self.done && deadline < Local::now()
        } else {
            false
        }
    }
}
