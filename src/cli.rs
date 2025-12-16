use crate::errors::Result;
use crate::io::export_data;
use crate::logic::{add_task, list_tasks, mark_task_done, parse_due_date, remove_task};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rustodo", about = "A simple CLI to-do application")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    #[command(about = "Add a new task")]
    Add {
        #[arg(required = true, num_args(1..), help = "Title of the task")]
        title: Vec<String>,

        #[arg(long = "due", help = "Optional due date in YYYY-MM-DD format")]
        due: Option<String>,
    },

    #[command(about = "Remove a task by its ID")]
    Remove {
        #[arg(required = true, help = "ID of the task to remove")]
        id: String,
    },

    #[command(about = "List all tasks")]
    List,

    #[command(about = "Mark a task as done by its ID")]
    Done {
        #[arg(required = true, help = "ID of the task to mark as done")]
        id: String,
    },

    #[command(about = "Export tasks to a specified format")]
    Export {
        #[arg(
            long = "format",
            default_value = "csv",
            help = "Export format: 'csv' or 'markdown'"
        )]
        format: String,
    },
}

pub fn exec(command: Command) -> Result<()> {
    match command {
        Command::Add { title, due } => {
            let due_dt = match due {
                Some(s) => Some(parse_due_date(&s)?),
                None => None,
            };
            add_task(&title.join(" "), due_dt)?
        }
        Command::Remove { id } => remove_task(&id)?,
        Command::List => list_tasks()?,
        Command::Done { id } => mark_task_done(&id)?,
        Command::Export { format } => export_data(&format)?,
    }
    Ok(())
}
