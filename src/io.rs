use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use crate::logic::Task;

const DATABASE: &str = "tasks.json";

pub fn read_tasks() -> Result<Vec<Task>, Box<dyn Error>> {
    let file = File::open(DATABASE)?;
    let reader = BufReader::new(file);
    let tasks = serde_json::from_reader(reader)?;
    Ok(tasks)
}

pub fn write_tasks(tasks: &Vec<Task>) -> Result<(), Box<dyn Error>> {
    let file = File::create(DATABASE)?;
    serde_json::to_writer_pretty(file, tasks)?;
    Ok(())
}