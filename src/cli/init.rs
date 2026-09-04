use std::{
    env,
    fs::{self, OpenOptions},
    io::{Result, Write},
};

pub fn create_file() -> Result<()> {
    let mut dir = env::current_dir()?;
    dir.push("waim.toml");

    fs::File::create_new(&dir)?;

    println!("Initialized a waim file at {}", &dir.to_string_lossy());

    Ok(())
}
