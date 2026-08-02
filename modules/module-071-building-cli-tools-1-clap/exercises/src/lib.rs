//! Module 071: Building CLI Tools I — `clap` Deep Dive — exercise scaffold.
//!
//! Build a task-manager CLI called `taskr` with three subcommands: `add`, `list`, and `done`.

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
        // TODO(module-071): add a value_parser constraint to reject id <= 0.
        // Hint: use `clap::value_parser!(u64).range(1..)` or a custom parser.
        id: u64,
    },
}

/// Execute the command and return a result string.
pub fn execute(_cli: &Cli) -> String {
    // TODO(module-071): match on `cli.command` and return:
    //   Commands::Add { description } => format!("Added: {description}")
    //   Commands::List => "Listing tasks...".to_string()
    //   Commands::Done { id } => format!("Completed task {id}")
    panic!("TODO(module-071): implement execute")
}
