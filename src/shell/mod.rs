use anyhow::Result;
use colored::*;
use std::io::{self, Write};

pub async fn run_interactive_shell() -> Result<()> {
    println!("{}", "🐚 Welcome to RCT Interactive Shell!".green().bold());
    println!("Type 'help' for commands, 'exit' to quit");
    println!("{}", "─".repeat(50).dimmed());
    
    loop {
        print!("{} ", "rct>".blue().bold());
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        match input {
            "exit" | "quit" => {
                println!("{} Goodbye! 👋", "✓".green());
                break;
            }
            "help" => {
                show_shell_help();
            }
            "clear" => {
                print!("\x1B[2J\x1B[1;1H");
            }
            "" => continue,
            _ => {
                println!("{} Unknown command: {}. Type 'help' for available commands.", 
                    "ℹ".blue(), input);
            }
        }
    }
    
    Ok(())
}

fn show_shell_help() {
    println!("{}", "Available Commands:".green().bold());
    println!("  {} - Show this help", "help".cyan());
    println!("  {} - Clear the screen", "clear".cyan());
    println!("  {} - Exit the shell", "exit/quit".cyan());
}