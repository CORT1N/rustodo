mod cli;
mod errors;
mod io;
mod logic;

use clap::Parser;

use logic::{add_task, list_tasks, mark_task_done, remove_task};
use cli::{ Cli, Command, exec };


fn main() {
    let cli = Cli::parse();
    exec(cli.command).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });
}