use clap::{Parser, Subcommand};
use std::fs::{read_to_string, OpenOptions};
use std::io::Write;

#[derive(Parser)]
#[command(name = "Todo")]
#[command(about = "A simple to-do list CLI app")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        #[arg()]
        task: String,
    },
    List,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { task } => add_task(task),
        Commands::List => list_tasks(),
    }
}

fn add_task(task: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("todo.txt")
        .expect("Failed to open file");

    writeln!(file, "{}", task).expect("Failed to write to file");
    println!("Added task: {}", task);
}

fn list_tasks() {
    let content = read_to_string("todo.txt").expect("Failed to read file");

    if content.is_empty() {
        println!("No tasks found.");
    } else {
        for (index, task) in content.lines().enumerate() {
            println!("{}: {}", index, task);
        }
    }
}