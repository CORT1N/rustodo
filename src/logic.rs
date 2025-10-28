use rand::distr::Alphanumeric;
use rand::Rng;
use serde::{Deserialize, Serialize};
use crate::io::{read_tasks, write_tasks};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
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

pub fn add_task(title: &str) {
    println!("Action: add, Arg: {}", title);
    let mut tasks = read_tasks().unwrap_or_else(|_| Vec::new());
    let task = Task::new(title.to_string());
    tasks.push(task.clone());
    write_tasks(&tasks).expect("Failed to write tasks to database");
    println!("Created task: {:?}", task);
}

pub fn remove_task(id: &str) {
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

pub fn list_tasks() {
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

pub fn mark_task_done(id: &str) {
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