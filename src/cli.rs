use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "termtask")]
#[command(about = "A stylish terminal-based todo application")]
#[command(version)]
#[command(long_about = "A stylish terminal-based todo application with support for project-specific and global todos")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    #[arg(long, global = true, help = "Force global todo storage instead of project-specific")]
    pub global: bool,

    #[arg(long, value_name = "PATH", help = "Path to custom todo.json file")]
    pub file: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Initialize project todos in the current directory")]
    Init {
        #[arg(long, help = "Create personal todos (.todo.json) instead of shared (todo.json)")]
        personal: bool,
    },
}

