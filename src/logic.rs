use rand::distr::Alphanumeric;
use rand::Rng;
use serde::{Deserialize, Serialize};
use crate::io::{read_tasks, write_tasks};
use crate::errors::{Result, TodoError};
use chrono::prelude::*;
use colored::Colorize;
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    id: String,
    title: String,
    completed: bool,
    due: Option<DateTime<Local>>,
}

impl Task {
    fn new(title: String, due: Option<DateTime<Local>>) -> Self {
        Task {
            id: Task::generate_id(),
            title,
            completed: false,
            due,
        }
    }

    fn generate_id() -> String {
        rand::rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect()
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn is_completed(&self) -> bool {
        self.completed
    }

    pub fn get_due(&self) -> Option<DateTime<Local>> {
        self.due
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = if self.completed { "✅" } else { "  " };
        let due_str = match self.due {
            Some(dt) => {
                let today = Local::now().date_naive();
                let due = dt.date_naive();
                let base = format!("due: {}", dt.format("%Y-%m-%d %H:%M"));
                let colored = if self.completed {
                    base.bright_black()
                } else if due < today {
                    base.red()
                } else if due == today {
                    base.yellow()
                } else {
                    base.normal()
                };
                format!("({})", colored)
            }
            None => "".to_string(),
        };
        write!(
            f,
            "{} {}   {} {:<16}",
            status,
            &self.id[..8.min(self.id.len())].bright_black(),
            self.title,
            due_str,
        )
    }
}

pub fn add_task(title: &str, due: Option<DateTime<Local>>) -> Result<()> {
    let mut tasks = read_tasks()?;
    let task = Task::new(title.to_string(), due);
    tasks.push(task.clone());
    write_tasks(&tasks)?;
    println!("{} {}", "Created task:".green(), task);
    Ok(())
}

pub fn remove_task(id: &str) -> Result<()> {
    let mut tasks = read_tasks()?;
    let removed: Vec<Task> = tasks.iter().filter(|task| task.id.starts_with(id)).cloned().collect();
    if removed.is_empty() {
        return Err(TodoError::TaskNotFound(id.to_string()));
    }
    tasks.retain(|task| !task.id.starts_with(id));
    write_tasks(&tasks)?;
    for task in removed {
        println!("{} {}", "Removed task:".green(), task);
    };
    Ok(())
}

pub fn list_tasks() -> Result<()> {
    let tasks = read_tasks()?;
    if tasks.is_empty() {
        println!("No tasks found.");
    } else {
        tasks.iter().for_each(|task| println!("{}", task));
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
            println!("{} {}", "Marked task as done:".green(), task);
        }
    }
    if found {
        write_tasks(&tasks)?;
        Ok(())
    } else {
        Err(TodoError::TaskNotFound(id.to_string()))
    }
}

pub fn parse_due_date(due_str: &str) -> Result<DateTime<Local>> {
    let naive_dt = NaiveDateTime::parse_from_str(due_str, "%Y-%m-%d %H:%M")
        .map_err(|_| TodoError::InvalidDueDate(due_str.to_string()))?;
    let local_dt: DateTime<Local> = Local.from_local_datetime(&naive_dt)
        .single()
        .ok_or_else(|| TodoError::InvalidDueDate(due_str.to_string()))?;
    Ok(local_dt)
}