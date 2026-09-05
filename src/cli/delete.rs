use super::add::Todo;
use std::{self, env, error::Error, fs};

pub fn delete_by_id(id: usize) -> Result<(), Box<dyn Error>> {
    let mut dir = env::current_dir()?;
    dir.push("waim.toml");

    let file = fs::read_to_string(&dir)?;

    let mut content: Todo = toml::from_str(&file)?;

    if let Some(ref mut tasks) = content.task {
        let init_len = tasks.len();

        tasks.retain(|task| task.id != id);

        if tasks.len() == init_len {
            println!("No task with ID {}", id);
            return Ok(());
        }

        println!("Deleted task {}", id);
    } else {
        println!("Missing task list");
        return Ok(());
    }

    let new_content = toml::to_string_pretty(&content)?;

    fs::write(dir, new_content)?;

    Ok(())
}
