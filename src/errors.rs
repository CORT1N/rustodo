use thiserror::Error;
use std::io;

#[derive(Error, Debug)]
pub enum TodoError {
    #[error("I/O Error: {0}")]
    Io(#[from] io::Error),

    #[error("Serialization/Deserialization Error: {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("Task with ID {0} not found.")]
    TaskNotFound(String),

    #[error("Invalid due date format: {0}. Expected format: YYYY-MM-DD HH:MM")]
    InvalidDueDate(String),
}

pub type Result<T> = std::result::Result<T, TodoError>;