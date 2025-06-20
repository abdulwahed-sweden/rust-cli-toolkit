use clap::Subcommand;
use anyhow::Result;
use colored::*;

#[derive(Subcommand)]
pub enum TaskCommands {
    /// List all tasks
    List,
    /// Add a new task
    Add {
        /// Task description
        description: String,
    },
    /// Mark task as complete
    Complete {
        /// Task ID
        id: u32,
    },
}

pub async fn task_handler(action: TaskCommands) -> Result<()> {
    match action {
        TaskCommands::List => {
            list_tasks().await
        }
        TaskCommands::Add { description } => {
            add_task(&description).await
        }
        TaskCommands::Complete { id } => {
            complete_task(id).await
        }
    }
}

async fn list_tasks() -> Result<()> {
    println!("{}", "📋 Task List".green().bold());
    println!("{}", "═".repeat(50).dimmed());
    // Placeholder implementation
    println!("No tasks yet. Use 'rct task add' to create one!");
    Ok(())
}

async fn add_task(description: &str) -> Result<()> {
    println!("{} Added task: '{}'", "✓".green(), description.cyan());
    Ok(())
}

async fn complete_task(id: u32) -> Result<()> {
    println!("{} Completed task #{}", "✓".green(), id.to_string().cyan());
    Ok(())
}