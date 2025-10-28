use clap::{Parser, Subcommand};
use crate::logic::{add_task, list_tasks, mark_task_done, remove_task};
use crate::errors::Result;

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

pub fn exec(command: Command) -> Result<()> {
    match command {
        Command::Add { title } => add_task(&title.join(" "))?,
        Command::Remove { id } => remove_task(&id)?,
        Command::List => list_tasks()?,
        Command::Done { id } => mark_task_done(&id)?,
    }
    Ok(())
}