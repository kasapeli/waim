mod cli;

use cli::{
    add::add_new,
    args::{Args, Cli, ListOpt},
    init::create_file,
    list::list_all,
};

use clap::Parser;
use std;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    match &args.command {
        Args::Init => {
            create_file()?;
        }
        Args::Add { task } => {
            add_new(task.to_string())?;
        }
        Args::List { opt } => match opt {
            ListOpt::All => {
                list_all()?;
            }
            ListOpt::Done => {}
            ListOpt::Undone => {}
        },
    }

    Ok(())
}
