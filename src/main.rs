mod cli;
mod git;
mod models;
mod prompt;
mod storage;
mod ui;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use git::GitRepository;
use prompt::{ProjectInitializer, TodoStorageChoice};
use ui::App;

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.explain {
        cli::explain_workflow();
        return Ok(());
    }

    match cli.command {
        Some(Commands::Init { personal }) => {
            handle_init_command(personal)?;
        }
        None => {
            let mut app = App::new_with_options(cli.global)?;
            app.run()?;
        }
    }

    Ok(())
}

fn handle_init_command(personal: bool) -> Result<()> {
    if let Some(repo) = GitRepository::find_repository() {
        if repo.has_todo_file().is_some() {
            println!("Todo file already exists in this project!");
            println!("   Existing file: {}", repo.has_todo_file().unwrap().display());
            return Ok(());
        }

        let choice = if personal {
            TodoStorageChoice::Personal
        } else {
            TodoStorageChoice::Project
        };

        let todo_path = ProjectInitializer::create_todo_file(&repo, choice.clone())?;
        
        match choice {
            TodoStorageChoice::Project => {
                println!("Created todo.json in project root");
                println!("✓ This file will be committed with your code");
                println!("✓ Team members can see and edit shared project tasks");
            }
            TodoStorageChoice::Personal => {
                println!("Created .todo.json (personal notes)");
                println!("✓ Added .todo.json to .gitignore");
                println!("✓ This stays private - not shared with team");
            }
            _ => {}
        }

        println!("File created: {}", todo_path.display());
        println!("Run 'termtask' to start managing your todos!");
    } else {
        println!("Not in a git repository!");
        println!("   Initialize command only works inside git projects.");
        println!("   Use 'termtask --global' for global todos.");
    }

    Ok(())
}