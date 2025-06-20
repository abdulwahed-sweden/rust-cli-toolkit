// src/commands/file.rs - Replace your current file with this
use clap::Subcommand;
use anyhow::Result;
use colored::*;
use std::fs;
use std::io::{self, Write};

#[derive(Subcommand)]
pub enum FileCommands {
    /// Create a new file
    Create {
        /// File path
        path: String,
        /// Content to write (optional)
        content: Option<String>,
    },
    /// Read and display file content
    Read {
        /// File path
        path: String,
        /// Show line numbers
        #[arg(short, long)]
        numbers: bool,
    },
    /// List files in directory
    List {
        /// Directory path
        #[arg(default_value = ".")]
        path: String,
    },
    /// Copy a file
    Copy {
        /// Source file
        source: String,
        /// Destination file
        dest: String,
    },
    /// Delete a file
    Delete {
        /// File path
        path: String,
    },
}

pub async fn file_handler(action: FileCommands) -> Result<()> {
    match action {
        FileCommands::Create { path, content } => {
            create_file(&path, content.as_deref()).await
        }
        FileCommands::Read { path, numbers } => {
            read_file(&path, numbers).await
        }
        FileCommands::List { path } => {
            list_files(&path).await
        }
        FileCommands::Copy { source, dest } => {
            copy_file(&source, &dest).await
        }
        FileCommands::Delete { path } => {
            delete_file(&path).await
        }
    }
}

async fn create_file(path: &str, content: Option<&str>) -> Result<()> {
    let content = match content {
        Some(c) => c.to_string(),
        None => {
            print!("Enter content for {}: ", path.cyan());
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            input.trim().to_string()
        }
    };
    
    fs::write(path, content)?;
    println!("{} File '{}' created successfully!", "✓".green(), path.cyan());
    Ok(())
}

async fn read_file(path: &str, numbers: bool) -> Result<()> {
    let content = fs::read_to_string(path)?;
    
    println!("{} Content of '{}':", "📄".blue(), path.cyan());
    println!("{}", "─".repeat(50).dimmed());
    
    if numbers {
        for (i, line) in content.lines().enumerate() {
            println!("{:4} │ {}", (i + 1).to_string().dimmed(), line);
        }
    } else {
        println!("{}", content);
    }
    
    Ok(())
}

async fn list_files(path: &str) -> Result<()> {
    println!("{} Files in '{}':", "📁".blue(), path.cyan());
    
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let filename = entry.file_name();
        let is_dir = entry.file_type()?.is_dir();
        let icon = if is_dir { "📁" } else { "📄" };
        println!("  {} {}", icon, filename.to_string_lossy());
    }
    
    Ok(())
}

async fn copy_file(source: &str, dest: &str) -> Result<()> {
    fs::copy(source, dest)?;
    println!("{} Copied '{}' to '{}'", "✓".green(), source.cyan(), dest.cyan());
    Ok(())
}

async fn delete_file(path: &str) -> Result<()> {
    print!("Are you sure you want to delete '{}'? (y/N): ", path.red());
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    if input.trim().to_lowercase().starts_with('y') {
        fs::remove_file(path)?;
        println!("{} File '{}' deleted", "✓".green(), path.cyan());
    } else {
        println!("{} Operation cancelled", "ℹ".blue());
    }
    
    Ok(())
}