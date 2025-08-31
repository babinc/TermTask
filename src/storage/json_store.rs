use crate::models::TodoList;
use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

pub struct JsonStore {
    file_path: PathBuf,
}

impl JsonStore {
    pub fn new(file_path: PathBuf) -> Self {
        Self { file_path }
    }

    pub fn load(&self) -> Result<TodoList> {
        if !self.file_path.exists() {
            return Ok(TodoList::new());
        }

        let content = fs::read_to_string(&self.file_path)
            .with_context(|| format!("Failed to read todo file: {}", self.file_path.display()))?;

        if content.trim().is_empty() {
            return Ok(TodoList::new());
        }

        let todo_list: TodoList = serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse todo file: {}", self.file_path.display()))?;

        Ok(todo_list)
    }

    pub fn save(&self, todo_list: &TodoList) -> Result<()> {
        if let Some(parent) = self.file_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
        }

        let content = serde_json::to_string_pretty(todo_list)
            .context("Failed to serialize todo list")?;

        fs::write(&self.file_path, content)
            .with_context(|| format!("Failed to write todo file: {}", self.file_path.display()))?;

        Ok(())
    }

    pub fn get_file_path(&self) -> &PathBuf {
        &self.file_path
    }

    pub fn get_default_path() -> Result<PathBuf> {
        let mut path = dirs::data_local_dir()
            .context("Could not determine local data directory")?;
        path.push("termtask");
        path.push("todos.json");
        Ok(path)
    }
}