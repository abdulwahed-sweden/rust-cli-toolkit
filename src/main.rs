// src/main.rs

#![allow(dead_code)]
#![allow(unused_imports)]


use clap::{Parser, Subcommand};
use anyhow::Result;

mod commands;
mod config;
mod shell;
mod utils;
mod errors;

#[derive(Parser)]
#[command(
    name = "rct",
    about = "Rust CLI Toolkit - A powerful command-line utility suite",
    version = "0.1.0"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    
    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize configuration and setup
    Init,
    /// File operations
    File {
        #[command(subcommand)]
        action: commands::FileCommands,
    },
    /// Network utilities
    Net {
        #[command(subcommand)]
        action: commands::NetCommands,
    },
    /// System information
    System {
        #[command(subcommand)]
        action: commands::SystemCommands,
    },
    /// Task management
    Task {
        #[command(subcommand)]
        action: commands::TaskCommands,
    },
    /// Configuration management
    Config {
        #[command(subcommand)]
        action: commands::ConfigCommands,
    },
    /// Interactive shell mode
    Shell,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    if cli.verbose {
        println!("🔧 Verbose mode enabled");
    }
    
    match cli.command {
        Some(Commands::Init) => {
            commands::init_handler().await?;
        }
        Some(Commands::File { action }) => {
            commands::file_handler(action).await?;
        }
        Some(Commands::Net { action }) => {
            commands::net_handler(action).await?;
        }
        Some(Commands::System { action }) => {
            commands::system_handler(action).await?;
        }
        Some(Commands::Task { action }) => {
            commands::task_handler(action).await?;
        }
        Some(Commands::Config { action }) => {
            commands::config_handler(action).await?;
        }
        Some(Commands::Shell) => {
            shell::run_interactive_shell().await?;
        }
        None => {
            println!("Welcome to Rust CLI Toolkit!");
            println!("Use 'rct --help' to see available commands");
            println!("Use 'rct init' to get started");
        }
    }
    
    Ok(())
}