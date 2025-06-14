use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{self, Write};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
}

fn main() {
    let mut tasks: Vec<Task> = load_tasks();

    loop {
        println!("\n To-do List Menu");
        println!("1. Add Task");
        println!("2. view Task");
        println!("3. mark Task as complete");
        println!("4. Delete Task");
        println!("5. Exit");

        let choice = get_input("Enter your choice ");
        match choice.trim() {
            "1" => add_task(&mut tasks),
            "2" => view_task(&mut tasks),
            "3" => mark_task_complete(&mut tasks),
            "4" => delete_task(&mut tasks),
            "5" => {
                save_task(&mut tasks);
                println!("task saved goodbye ");
                break;
            }
            _ => println!("Invalid choice. pls try again "),
        }
    }
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");

    input
}

fn load_tasks() -> Vec<Task> {
    match fs::read_to_string("tasks.json") {
        Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| Vec::new()),
        Err(_) => Vec::new(),
    }
}

fn save_task(tasks: &Vec<Task>) {
    let json = serde_json::to_string_pretty(tasks).expect("Failed to serialize task");
    let mut file = File::create("tasks.json").expect("Failed to save tasks to file");
    file.write_all(json.as_bytes())
        .expect("failed to write tasks to file ");
}

fn add_task(tasks: &mut Vec<Task>) {
    let description = get_input("Enter task description");
    let id = tasks.len() + 1;
    tasks.push(Task {
        id,
        description: description.trim().to_string(),
        completed: false,
    });
    println!("Task added");
}

fn view_task(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("No task found ");
    } else {
        for task in tasks {
            let status = if task.completed { "✅" } else { "❌" };
            println!("{} - {}: {}", task.id, status, task.description);
        }
    }
}

fn mark_task_complete(tasks: &mut Vec<Task>) {
    let id = get_input("Enter  task Id to mark as completed:");
    if let Ok(id) = id.trim().parse::<usize>() {
        if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
            task.completed = true;
            println!("Task marked as completed ");
        } else {
            println!("Task not found ");
        }
    } else {
        println!("Invalid task Id")
    }
}

fn delete_task(tasks: &mut Vec<Task>) {
    let id = get_input("Enter task ID to delete :");
    if let Ok(id) = id.trim().parse::<usize>() {
        if let Some(index) = tasks.iter().position(|t| t.id == id) {
            tasks.remove(index);
            println!("Task deleted");
        } else {
            println!("Task not found")
        }
    } else {
        println!("Invalid task id.")
    }
}
read