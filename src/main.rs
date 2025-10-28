use std::env;

fn check_args_length(command: &str, args: &Vec<String>) {}

fn add_task() {}

fn remove_task() {}

fn list_tasks() {}

fn mark_task_done() {}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { panic!("Usage: rustodo <command>"); }
    let command = args[1].as_str();
    match command {
        "add" => {
            if args.len() < 3 {
                panic!("Usage: rustodo add <title of your task>");
            }
            println!("Action: {}, Arg: {}", command, args[2..].to_vec().join(" "));
        }
        "remove" => {
            if args.len() < 3 {
                panic!("Usage: rustodo remove <ID of your task>");
            }
            println!("Action: {}, Arg: {}", command, args[2]);
        }
        "list" => println!("Action: {}", command),
        "done" => {
            if args.len() < 3 {
                panic!("Usage: rustodo done <ID of your task>");
            }
            println!("Action: {}, Arg: {}", command, args[2]);
        }
        _ => println!("Unknown command"),
    }
}