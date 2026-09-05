use super::add::Todo;
use std::{env, error::Error, fs};

pub fn toggle_by_id(id: usize) -> Result<(), Box<dyn Error>> {
    let mut dir = env::current_dir()?;
    dir.push("waim.toml");

    let file = fs::read_to_string(&dir)?;

    let mut content: Todo = toml::from_str(&file)?;

    if let Some(tasks) = content.task.as_mut() {
        if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
            task.completion = !task.completion;

            let status = if task.completion { "done" } else { "undone" };
            println!("Marked task {} as {}", id, status);
        } else {
            println!("No task with ID {}", id);
            return Ok(());
        }
    } else {
        println!("Missing task list");
        return Ok(());
    }

    let new_content = toml::to_string_pretty(&content)?;

    fs::write(&dir, new_content)?;

    Ok(())
}
