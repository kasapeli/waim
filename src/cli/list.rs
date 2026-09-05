use super::add::Todo;

use std::{env, error::Error, fs};

pub fn list_all() -> Result<(), Box<dyn Error>> {
    let mut dir = env::current_dir()?;
    dir.push("waim.toml");

    let read = fs::read_to_string(dir)?;
    let content: Todo = toml::from_str(&read)?;

    if let Some(tasks) = content.task {
        let all_tasks = tasks.len();
        let mut completed_count = 0;

        for task in &tasks {
            if task.completion {
                completed_count += 1;
            }

            let status = if task.completion { "X" } else { "" };
            println!("[{:<1}] | {:^3} | {:>5}", status, task.id, task.task_name);
        }

        println!();
        println!("All tasks: {}", all_tasks);
        println!("Completed: {}", completed_count);
    }

    Ok(())
}

pub fn list_done() -> Result<(), Box<dyn Error>> {
    let mut dir = env::current_dir()?;
    dir.push("waim.toml");

    let read = fs::read_to_string(dir)?;
    let content: Todo = toml::from_str(&read)?;

    if let Some(tasks) = content.task {
        for task in &tasks {
            if task.completion {
                println!(" {:^3} | {:>5}", task.id, task.task_name);
            }
        }
    }

    Ok(())
}

pub fn list_undone() -> Result<(), Box<dyn Error>> {
    let mut dir = env::current_dir()?;
    dir.push("waim.toml");

    let read = fs::read_to_string(dir)?;
    let content: Todo = toml::from_str(&read)?;

    if let Some(tasks) = content.task {
        for task in &tasks {
            if !task.completion {
                println!(" {:^3} | {:>5}", task.id, task.task_name);
            }
        }
    }

    Ok(())
}
