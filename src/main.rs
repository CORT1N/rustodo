use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use rand::distr::Alphanumeric;
use rand::Rng;
use serde::{Deserialize, Serialize};

const DATABASE: &str = "tasks.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
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

fn read_tasks() -> Result<Vec<Task>, Box<dyn Error>> {
    let file = File::open(DATABASE)?;
    let reader = BufReader::new(file);
    let tasks = serde_json::from_reader(reader)?;
    Ok(tasks)
}

fn write_tasks(tasks: &Vec<Task>) -> Result<(), Box<dyn Error>> {
    let file = File::create(DATABASE)?;
    serde_json::to_writer_pretty(file, tasks)?;
    Ok(())
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
    let mut tasks = read_tasks().unwrap_or_else(|_| Vec::new());
    let task = Task::new(title.to_string());
    tasks.push(task.clone());
    write_tasks(&tasks).expect("Failed to write tasks to database");
    println!("Created task: {:?}", task);
}

fn remove_task(id: &str) {
    println!("Action: remove, Arg: {}", id);
    let mut tasks = read_tasks().unwrap_or_else(|_| Vec::new());
    let len = tasks.len();
    tasks.retain(|task| !task.id.starts_with(id));
    if tasks.len() == len {
        println!("No task found with ID: {}", id);
    } else {
        write_tasks(&tasks).expect("Failed to write tasks to database");
        println!("Removed task with ID: {}", id);
    }
}

fn list_tasks() {
    let tasks = read_tasks().unwrap_or_else(|_| Vec::new());
    if tasks.is_empty() {
        println!("No tasks found.");
    } else {
        const ID_LEN: usize = 8;
        tasks.iter().for_each(|task| {
            let status = if task.completed { "✅" } else { "  " };
            println!(
                "{} {}     {}",
                status,
                &task.id[..ID_LEN.min(task.id.len())],
                task.title,
            );
        });
    }
}

fn mark_task_done(id: &str) {
    println!("Action: done, Arg: {}", id);
    let mut tasks = read_tasks().unwrap_or_else(|_| Vec::new());
    let mut found = false;
    for task in &mut tasks {
        if task.id.starts_with(id) {
            task.completed = true;
            found = true;
            println!("Marked task as done: {:?}", task);
        }
    }
    if found {
        write_tasks(&tasks).expect("Failed to write tasks to database");
    } else {
        println!("No task found with ID: {}", id);
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