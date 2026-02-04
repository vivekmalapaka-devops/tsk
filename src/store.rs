use crate::todo::Todo;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Store {
    pub version: u32,
    pub next_id: u32,
    pub todos: Vec<Todo>,
}

impl Default for Store {
    fn default() -> Self {
        Self {
            version: 1,
            next_id: 1,
            todos: Vec::new(),
        }
    }
}

impl Store {
    pub fn load() -> io::Result<Self> {
        let path = Self::path()?;

        if !path.exists() {
            return Ok(Self::default());
        }

        let content = fs::read_to_string(&path)?;
        let store: Store = serde_json::from_str(&content)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        Ok(store)
    }

    pub fn save(&self) -> io::Result<()> {
        let path = Self::path()?;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(self)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        fs::write(&path, content)?;
        Ok(())
    }

    fn path() -> io::Result<PathBuf> {
        let home = dirs::home_dir()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Home directory not found"))?;

        Ok(home.join(".tsk").join("todos.json"))
    }

    pub fn add(&mut self, mut todo: Todo) -> &Todo {
        todo.id = self.next_id;
        self.next_id += 1;
        self.todos.push(todo);
        self.todos.last().unwrap()
    }

    pub fn get_mut(&mut self, id: u32) -> Option<&mut Todo> {
        self.todos.iter_mut().find(|t| t.id == id)
    }

    pub fn remove(&mut self, id: u32) -> Option<Todo> {
        if let Some(pos) = self.todos.iter().position(|t| t.id == id) {
            Some(self.todos.remove(pos))
        } else {
            None
        }
    }

    pub fn clear_completed(&mut self) -> usize {
        let before = self.todos.len();
        self.todos.retain(|t| !t.done);
        before - self.todos.len()
    }

    pub fn open_todos(&self) -> impl Iterator<Item = &Todo> {
        self.todos.iter().filter(|t| !t.done)
    }

    pub fn completed_todos(&self) -> impl Iterator<Item = &Todo> {
        self.todos.iter().filter(|t| t.done)
    }
}
