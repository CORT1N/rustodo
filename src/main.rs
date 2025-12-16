mod cli;
mod errors;
mod io;
mod logic;

use clap::Parser;
use colored::Colorize;

use cli::{Cli, exec};

fn main() {
    let cli = Cli::parse();
    exec(cli.command).unwrap_or_else(|err| {
        eprintln!("{} {}", "Error:".red(), err);
        std::process::exit(1);
    });
}