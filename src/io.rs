use crate::errors::Result;
use crate::logic::Task;
use chrono::Local;
use colored::Colorize;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

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

fn export_to_csv() -> Result<()> {
    let path = format!("export_{}.csv", suffix_export());
    let mut wtr = csv::Writer::from_path(&path)?;
    for task in read_tasks()? {
        wtr.serialize(task)?;
    }
    wtr.flush()?;
    Ok(())
}

fn export_to_markdown() -> Result<()> {
    let path = format!("export_{}.md", suffix_export());
    let mut file = File::create(&path)?;
    use std::io::Write;
    writeln!(file, "# To-Do List\n")?;
    for task in read_tasks()? {
        let status = if task.is_completed() { "~~" } else { "" };
        writeln!(
            file,
            "- {}{}{} (Due: {})",
            status,
            task.get_title(),
            status,
            task.get_due().map_or("-".to_string(), |d| d.to_string())
        )?;
    }
    Ok(())
}

pub fn export_data(format: &str) -> Result<()> {
    match format {
        "csv" => export_to_csv()?,
        "md" | "markdown" => export_to_markdown()?,
        _ => {
            println!(
                "{} {} {} {}",
                "Unsupported export format:".red(),
                format,
                "- Supported formats are 'csv' and 'markdown'.",
                "Exporting to CSV by default.".yellow()
            );
            export_to_csv()?;
        }
    }
    Ok(())
}

fn suffix_export() -> String {
    let now = Local::now();
    now.format("%Y-%m-%d_%H-%M-%S").to_string()
}
