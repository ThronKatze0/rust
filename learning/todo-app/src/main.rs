use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

use crate::todos::TodoList;
use clap::{Args, Parser, Subcommand};

mod todos;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// list all todos
    List,
    /// delete todos
    Delete { name: String },
    /// Add a todo
    Add { name: String, checked: Option<bool> },
    /// check a todo
    Check { name: String },
}

const FILE: &'static str = "./data.json";

fn main() {
    let cli = Cli::parse();
    let mut file_contents = String::new();
    File::open(FILE).expect("your filesystem no good").read_to_string(&mut file_contents).expect("Cannot read this shit");
    let mut todos: TodoList = if let Ok(result) = serde_json::from_str(&file_contents) {
        result
    } else {
        TodoList::new()
    };
    match cli.command {
        Command::List => println!("{:?}", todos.get_todos()),
        Command::Delete { name } => {
            if let Ok(_) = todos.remove(&name) {
                println!("Removed successfully");
            } else {
                println!("Todo {} is not in todo list", name)
            }
        }
        Command::Add { name, checked } => {
            if let Ok(_) = todos.add(
                name,
                if let Some(checked) = checked {
                    checked
                } else {
                    false
                },
            ) {
                println!("Added successfully");
            } else {
                println!("Todo already exists");
            }
        }
        Command::Check { name } => {
            if let Ok(_) = todos.check(&name) {
                println!("Checked successfully");
            } else {
                println!("Todo {} not found", name);
            }
        }
    }
    let json_text: String = serde_json::to_string(&todos).expect("What the fuck happened?");
    println!("{}", json_text);
    File::create(FILE).expect("mhm").write_all(&json_text.as_bytes()).expect("writing went wrong");
}
