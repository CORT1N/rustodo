use rand::distr::Alphanumeric;
use rand::Rng;
use serde::{Deserialize, Serialize};
use crate::io::{read_tasks, write_tasks};
use crate::errors::{Result, TodoError};

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

pub fn add_task(title: &str) -> Result<()> {
    let mut tasks = read_tasks()?;
    let task = Task::new(title.to_string());
    tasks.push(task.clone());
    write_tasks(&tasks)?;
    println!("Created task: {:?}", task);
    Ok(())
}

pub fn remove_task(id: &str) -> Result<()> {
    let mut tasks = read_tasks()?;
    let len = tasks.len();
    tasks.retain(|task| !task.id.starts_with(id));
    if tasks.len() == len {
        Err(TodoError::TaskNotFound(id.to_string()))
    } else {
        write_tasks(&tasks)?;
        println!("Removed task with ID: {}", id);
        Ok(())
    }
}

pub fn list_tasks() -> Result<()> {
    let tasks = read_tasks()?;
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
    Ok(())
}

pub fn mark_task_done(id: &str) -> Result<()> {
    let mut tasks = read_tasks()?;
    let mut found = false;
    for task in &mut tasks {
        if task.id.starts_with(id) {
            task.completed = true;
            found = true;
            println!("Marked task as done: {:?}", task);
        }
    }
    if found {
        write_tasks(&tasks)?;
        Ok(())
    } else {
        Err(TodoError::TaskNotFound(id.to_string()))
    }
}