use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use crate::errors::Result;
use crate::logic::Task;

const DATABASE: &str = "tasks.json";

fn init_db() -> Result<()> {
    if !Path::new(DATABASE).exists() {
        let file = File::create(DATABASE)?;
        serde_json::to_writer_pretty(file, &Vec::<Task>::new())?;
    }
    Ok(())
}

pub fn read_tasks() -> Result<Vec<Task>> {
    init_db()?;
    let file = File::open(DATABASE)?;
    let reader = BufReader::new(file);
    let tasks = serde_json::from_reader(reader)?;
    Ok(tasks)
}

pub fn write_tasks(tasks: &Vec<Task>) -> Result<()> {
    let file = File::create(DATABASE)?;
    serde_json::to_writer_pretty(file, tasks)?;
    Ok(())
}