pub mod file;
pub mod net;
pub mod system;
pub mod task;
pub mod config;

use clap::Subcommand;
use anyhow::Result;
use colored::*;

pub use file::*;
pub use net::*;
pub use system::*;
pub use task::*;
pub use config::*;

pub async fn init_handler() -> Result<()> {
    println!("{} Initializing Rust CLI Toolkit...", "🚀".blue());
    
    // Create config directory
    let config_dir = dirs::config_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("rust-cli-toolkit");
    
    if !config_dir.exists() {
        std::fs::create_dir_all(&config_dir)?;
        println!("{} Created config directory: {}", "✓".green(), config_dir.display());
    }
    
    // Create data directory
    let data_dir = dirs::data_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("rust-cli-toolkit");
    
    if !data_dir.exists() {
        std::fs::create_dir_all(&data_dir)?;
        println!("{} Created data directory: {}", "✓".green(), data_dir.display());
    }
    
    println!("{} Initialization complete!", "🎉".green());
    println!("Run {} to start using the toolkit", "rct --help".cyan());
    
    Ok(())
}