use clap::{Parser, Subcommand};
use crate::logic::{add_task, list_tasks, mark_task_done, remove_task, parse_due_date};
use crate::io::export_data;
use crate::errors::Result;

#[derive(Parser)]
#[command(name = "rustodo", about = "A simple CLI to-do application")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Add {
        #[arg(required = true, num_args(1..))]
        title: Vec<String>,

        #[arg(long= "due")]
        due: Option<String>,
    },

    Remove {
        #[arg(required = true)]
        id: String,
    },

    List,

    Done {
        #[arg(required = true)]
        id: String,
    },

    Export {
        #[arg(long="format", default_value = "csv")]
        format: String,
    }
}

pub fn exec(command: Command) -> Result<()> {
    match command {
        Command::Add { title, due } => {
            let due_dt = match due {
                Some(s) => Some(parse_due_date(&s)?),
                None => None,
            };
            add_task(&title.join(" "), due_dt)?
        },
        Command::Remove { id } => remove_task(&id)?,
        Command::List => list_tasks()?,
        Command::Done { id } => mark_task_done(&id)?,
        Command::Export { format } => export_data(&format)?,
    }
    Ok(())
}