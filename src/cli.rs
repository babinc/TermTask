use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "termtask")]
#[command(about = "A stylish terminal-based todo application")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Force global todo storage instead of project-specific
    #[arg(long, global = true)]
    pub global: bool,

    /// Show workflow explanation without starting the app
    #[arg(long)]
    pub explain: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize project todos in the current directory
    Init {
        /// Create personal todos (.todo.json) instead of shared (todo.json)
        #[arg(long)]
        personal: bool,
    },
}

pub fn explain_workflow() {
    println!();
    println!("TermTask Workflow Guide");
    println!("=========================");
    println!();
    println!("Project-Based Todo Management:");
    println!();
    println!("When you run termtask inside a git repository, you get three options:");
    println!();
    println!("1. Project Todos (todo.json) - RECOMMENDED");
    println!("   • File committed to git repository");  
    println!("   • Shared with your entire team");
    println!("   • Perfect for: sprint planning, feature lists, bug tracking");
    println!("   • Everyone sees the same project tasks");
    println!();
    println!("2. Personal Notes (.todo.json)");
    println!("   • File added to .gitignore (stays private)");
    println!("   • Only visible to you");
    println!("   • Perfect for: learning notes, personal reminders, temp tasks");
    println!("   • Team members won't see your private thoughts");
    println!();
    println!("3. Global Todos");
    println!("   • Stored in ~/.local/share/termtask/todos.json");
    println!("   • Works across all projects and directories");
    println!("   • Perfect for: cross-project tasks, general life todos");
    println!("   • Not tied to any specific project");
    println!();
    println!("Commands:");
    println!("   termtask              Start the app (auto-detects git projects)");
    println!("   termtask init         Initialize shared project todos");
    println!("   termtask init --personal  Initialize private project todos");
    println!("   termtask --global     Force global mode (ignore project detection)");
    println!("   termtask --explain    Show this help");
    println!();
    println!("Tips:");
    println!("   • Most teams should use 'todo.json' for shared project planning");
    println!("   • Use '.todo.json' for personal notes you don't want to share");
    println!("   • Global todos are great for cross-project tasks");
    println!("   • The app auto-detects your context and prompts appropriately");
    println!();
}