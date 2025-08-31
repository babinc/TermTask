use std::env;
use std::path::{Path, PathBuf};

pub struct GitRepository {
    pub root: PathBuf,
}

impl GitRepository {
    pub fn find_repository() -> Option<Self> {
        let current_dir = env::current_dir().ok()?;
        Self::find_git_root(&current_dir).map(|root| Self { root })
    }

    fn find_git_root(start_dir: &Path) -> Option<PathBuf> {
        let mut current = start_dir;
        
        loop {
            let git_dir = current.join(".git");
            if git_dir.exists() {
                return Some(current.to_path_buf());
            }
            
            current = current.parent()?;
        }
    }

    pub fn has_todo_file(&self) -> Option<PathBuf> {
        let todo_json = self.root.join("todo.json");
        if todo_json.exists() {
            return Some(todo_json);
        }

        let hidden_todo = self.root.join(".todo.json");
        if hidden_todo.exists() {
            return Some(hidden_todo);
        }

        None
    }

    pub fn get_project_todo_path(&self) -> PathBuf {
        self.root.join("todo.json")
    }

    pub fn get_personal_todo_path(&self) -> PathBuf {
        self.root.join(".todo.json")
    }

    pub fn get_gitignore_path(&self) -> PathBuf {
        self.root.join(".gitignore")
    }

    pub fn add_to_gitignore(&self, entry: &str) -> Result<(), std::io::Error> {
        use std::fs::{read_to_string, write};
        use std::io::ErrorKind;

        let gitignore_path = self.get_gitignore_path();
        
        let mut content = match read_to_string(&gitignore_path) {
            Ok(content) => content,
            Err(e) if e.kind() == ErrorKind::NotFound => String::new(),
            Err(e) => return Err(e),
        };

        if !content.contains(entry) {
            if !content.is_empty() && !content.ends_with('\n') {
                content.push('\n');
            }
            content.push_str(entry);
            content.push('\n');
            write(&gitignore_path, content)?;
        }

        Ok(())
    }
}