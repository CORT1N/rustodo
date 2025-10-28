mod cli;
mod io;
mod logic;

use clap::Parser;

use logic::{add_task, list_tasks, mark_task_done, remove_task};
use cli::{ Cli, Command };


fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Add { title } => { add_task(&title.join(" ")) }
        Command::Remove { id } => { remove_task(&id) }
        Command::List => { list_tasks() }
        Command::Done { id } => { mark_task_done(&id) }
    }
}