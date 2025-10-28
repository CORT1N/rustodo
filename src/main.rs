mod io;
mod logic;

use std::env;

use logic::{add_task, list_tasks, mark_task_done, remove_task};

fn check_args_length(command: &str, args: &Vec<String>) {
    let usage = match command {
        "add" => "<title of your task>",
        "remove" | "done" => "<ID of your task>",
        _ => "",
    };
    match command {
        "add" | "remove" | "done" => {
            if args.len() < 3 { panic!("Usage: rustodo {} {}", command, usage); }
        }
        _ => (),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { panic!("Usage: rustodo <command>"); }
    let command = args[1].as_str();
    check_args_length(command, &args);
    match command {
        "add" => { add_task(&args[2..].to_vec().join(" ")) }
        "remove" => { remove_task(&args[2]) }
        "list" => { list_tasks() }
        "done" => { mark_task_done(&args[2]) }
        _ => println!("Unknown command"),
    }
}