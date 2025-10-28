mod cli;
mod errors;
mod io;
mod logic;

use clap::Parser;

use cli::{ Cli, exec };


fn main() {
    let cli = Cli::parse();
    exec(cli.command).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });
}