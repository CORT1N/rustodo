use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rustodo", about = "A simple CLI to-do application")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Add a new task
    Add {
        /// Title of the task
        #[arg(required = true, num_args(1..))]
        title: Vec<String>,
    },

    /// Remove a task by ID
    Remove {
        /// ID of the task
        #[arg(required = true)]
        id: String,
    },

    /// List all tasks
    List,

    /// Mark a task as done by ID
    Done {
        /// ID of the task
        #[arg(required = true)]
        id: String,
    },
}