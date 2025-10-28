use std::env;

use rand::distr::Alphanumeric;
use rand::Rng;

#[derive(Debug)]
struct Task {
    id: String,
    title: String,
    completed: bool,
}

impl Task {
    fn new(title: String) -> Self {
        Task {
            id: Task::generate_id(),
            title,
            completed: false,
        }
    }

    fn generate_id() -> String {
        rand::rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect()
    }
}

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

fn add_task(title: &str) {
    println!("Action: add, Arg: {}", title);
    let task = Task::new(title.to_string());
    println!("Created task: {:?}", task);
}

fn remove_task(id: &str) {
    println!("Action: remove, Arg: {}", id);
}

fn list_tasks() {
    println!("Action: list");
}

fn mark_task_done(id: &str) {
    println!("Action: done, Arg: {}", id)
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