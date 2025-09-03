use crate::git::GitRepository;
use std::io::{self, Write};

#[derive(Debug, Clone, PartialEq)]
pub enum TodoStorageChoice {
    Project,
    Personal,
    Global,
}

pub struct ProjectInitializer;

impl ProjectInitializer {
    pub fn prompt_for_initialization(repo: &GitRepository) -> Option<TodoStorageChoice> {
        println!();
        println!("Detected git repository: {}", repo.root.display());
        println!("No project todos found.");
        println!();
        println!("You have 3 options for managing todos in this project:");
        println!();

        println!("[1] Project todos (todo.json) - RECOMMENDED");
        println!("    • Shared with your team via git");
        println!("    • Committed to repository");
        println!("    • Everyone sees the same project tasks");
        println!("    • Great for: feature planning, bug lists, project milestones");
        println!();

        println!("[2] Personal project notes (.todo.json)");
        println!("    • Private to you (added to .gitignore)");
        println!("    • Not shared with team");
        println!("    • Great for: personal reminders, learning notes, temp tasks");
        println!();

        println!("[3] Global todos (continue current behavior)");
        println!("    • Stored in ~/.local/share/termtask/todos.json");
        println!("    • Cross-project tasks and general todos");
        println!("    • Not tied to any specific project");
        println!();

        loop {
            print!("Choice [1/2/3]: ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    match input.trim() {
                        "1" => {
                            Self::show_project_choice_explanation();
                            return Some(TodoStorageChoice::Project);
                        }
                        "2" => {
                            Self::show_personal_choice_explanation();
                            return Some(TodoStorageChoice::Personal);
                        }
                        "3" => {
                            Self::show_global_choice_explanation();
                            return Some(TodoStorageChoice::Global);
                        }
                        "" => {
                            Self::show_project_choice_explanation();
                            return Some(TodoStorageChoice::Project);
                        }
                        _ => {
                            println!("Please enter 1, 2, or 3 (or press Enter for default option 1)");
                            continue;
                        }
                    }
                }
                Err(_) => {
                    println!("Failed to read input. Using default option 1.");
                    Self::show_project_choice_explanation();
                    return Some(TodoStorageChoice::Project);
                }
            }
        }
    }

    fn show_project_choice_explanation() {
        println!();
        println!("Creating todo.json in project root...");
        println!("✓ This file will be committed with your code");
        println!("✓ Team members can see and edit shared project tasks");
        println!("✓ Perfect for tracking features, bugs, and project goals");
        println!();
        println!("Tip: Use personal notes (.todo.json) for private thoughts!");
        println!();
    }

    fn show_personal_choice_explanation() {
        println!();
        println!("Creating .todo.json (personal notes)...");
        println!("✓ Adding .todo.json to .gitignore (if it exists)");
        println!("✓ This stays private - not shared with team");
        println!("✓ Great for personal learning notes and reminders");
        println!();
        println!("Tip: Use 'todo.json' for tasks the whole team should see!");
        println!();
    }

    fn show_global_choice_explanation() {
        println!();
        println!("Continuing with global todos...");
        println!("✓ Using ~/.local/share/termtask/todos.json");
        println!("✓ These todos work across all projects");
        println!("✓ Run with --init flag later to set up project todos");
        println!();
    }

    pub fn create_todo_file(repo: &GitRepository, choice: TodoStorageChoice) -> Result<std::path::PathBuf, anyhow::Error> {
        use crate::models::TodoList;
        use crate::storage::JsonStore;

        let todo_path = match choice {
            TodoStorageChoice::Project => repo.get_project_todo_path(),
            TodoStorageChoice::Personal => {
                let path = repo.get_personal_todo_path();
                let _ = repo.add_to_gitignore(".todo.json");
                path
            }
            TodoStorageChoice::Global => {
                return Err(anyhow::anyhow!("Global choice should not create a file"));
            }
        };

        let store = JsonStore::new(todo_path.clone());
        let empty_list = TodoList::new();
        store.save(&empty_list)?;

        Ok(todo_path)
    }
}