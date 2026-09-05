use std::{
    env,
    fs::{self, OpenOptions},
    io::{self, BufRead, BufReader, Write},
    path::Path,
};

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct WaimFile {
    pub id: usize,
    pub task_name: String,
    pub completion: bool,
}

#[derive(Deserialize)]
pub struct Todo {
    pub task: Option<Vec<WaimFile>>,
}

pub fn add_new(task: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut dir = env::current_dir()?;
    dir.push("waim.toml");

    let id = if std::path::Path::new(&dir).exists() {
        let content = fs::read_to_string(&dir)?;
        let todo: Todo = toml::from_str(&content)?;

        todo.task
            .unwrap_or_default()
            .iter()
            .map(|t| t.id)
            .max()
            .unwrap_or(0)
            + 1
    } else {
        1
    };

    let waim_content = WaimFile {
        id: id,
        task_name: task,
        completion: false,
    };

    let file = OpenOptions::new().append(true).create(true).open(dir)?;

    if id > 1 {
        writeln!(&file, "")?;
    }
    writeln!(&file, "[[task]]")?;
    writeln!(&file, "id = {}", waim_content.id)?;
    writeln!(&file, "task_name = \"{}\"", waim_content.task_name)?;
    writeln!(&file, "completion = {}", waim_content.completion)?;

    Ok(())
}
