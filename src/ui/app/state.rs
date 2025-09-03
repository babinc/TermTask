use super::App;
use crate::git::GitRepository;
use crate::models::TodoList;
use crate::prompt::{ProjectInitializer, TodoStorageChoice};
use crate::storage::JsonStore;
use anyhow::Result;
use std::io::{self, Write};

impl App {
    pub(super) fn initialize_storage(force_global: bool, custom_path: Option<String>) -> Result<(JsonStore, TodoList)> {
        if let Some(path) = custom_path {
            let path_buf = std::path::PathBuf::from(&path);
            if !path_buf.exists() {
                if Self::prompt_create_custom_file(&path)? {
                    let store = JsonStore::new(path_buf);
                    let todos = TodoList::new();
                    store.save(&todos)?;
                    return Ok((store, todos));
                } else {
                    std::process::exit(0);
                }
            }
            let store = JsonStore::new(path_buf);
            let todos = store.load()?;
            return Ok((store, todos));
        }

        if force_global {
            let store = JsonStore::new(JsonStore::get_default_path()?);
            let todos = store.load()?;
            return Ok((store, todos));
        }

        if let Some(repo) = GitRepository::find_repository() {
            if let Some(existing_todo_path) = repo.has_todo_file() {
                let store = JsonStore::new(existing_todo_path);
                let todos = store.load()?;
                return Ok((store, todos));
            }

            if let Some(choice) = ProjectInitializer::prompt_for_initialization(&repo) {
                match choice {
                    TodoStorageChoice::Global => {
                        let store = JsonStore::new(JsonStore::get_default_path()?);
                        let todos = store.load()?;
                        Ok((store, todos))
                    }
                    TodoStorageChoice::Project | TodoStorageChoice::Personal => {
                        let todo_path = ProjectInitializer::create_todo_file(&repo, choice)?;
                        let store = JsonStore::new(todo_path);
                        let todos = store.load()?;
                        Ok((store, todos))
                    }
                }
            } else {
                let store = JsonStore::new(JsonStore::get_default_path()?);
                let todos = store.load()?;
                Ok((store, todos))
            }
        } else {
            let store = JsonStore::new(JsonStore::get_default_path()?);
            let todos = store.load()?;
            Ok((store, todos))
        }
    }

    fn prompt_create_custom_file(path: &str) -> Result<bool> {
        println!();
        println!("Todo file does not exist: {}", path);
        println!("Would you like to create it? [y/N]: ");

        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim().to_lowercase();
        Ok(input == "y" || input == "yes")
    }

    pub fn save_todos(&self) -> Result<()> {
        self.json_store.save(&self.todos)
    }

    pub fn save_config(&self) -> Result<()> {
        self.config_store.save(&self.config)
    }

    pub fn get_storage_context_display(&self) -> String {
        let store_path = self.json_store.get_file_path();

        if let Some(home_dir) = dirs::home_dir() {
            let global_path = home_dir.join(".local/share/termtask/todos.json");
            if *store_path == global_path {
                return "Global".to_string();
            }
        }

        if let Some(repo) = GitRepository::find_repository() {
            let project_todo = repo.root.join("todo.json");
            let personal_todo = repo.root.join(".todo.json");

            if *store_path == project_todo {
                let project_name = repo.root
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("Unknown");
                return format!("Project: {}", project_name);
            }

            if *store_path == personal_todo {
                let project_name = repo.root
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("Unknown");
                return format!("Personal: {}", project_name);
            }
        }

        if let Some(file_name) = store_path.file_name().and_then(|name| name.to_str()) {
            format!("Custom: {}", file_name)
        } else {
            "Unknown".to_string()
        }
    }
}