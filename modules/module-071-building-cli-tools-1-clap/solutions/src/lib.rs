//! Module 071: Building CLI Tools I — `clap` Deep Dive — reference solution.

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "taskr", about = "A simple task manager")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Add a new task
    Add {
        /// Task description
        description: String,
    },
    /// List all tasks
    List,
    /// Mark a task as done by ID
    Done {
        /// Task ID (must be greater than 0)
        #[arg(value_parser = clap::value_parser!(u64).range(1..))]
        id: u64,
    },
}

/// Execute the command and return a result string.
pub fn execute(cli: &Cli) -> String {
    match &cli.command {
        Commands::Add { description } => format!("Added: {description}"),
        Commands::List => "Listing tasks...".to_string(),
        Commands::Done { id } => format!("Completed task {id}"),
    }
}
