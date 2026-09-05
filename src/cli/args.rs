use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Args,
}

#[derive(Subcommand)]
pub enum Args {
    /// Initializes WAIM in the current directory
    Init,

    /// Adds a task to WAIM
    Add { task: String },

    /// Lists current tasks (options: all, done, undone)
    List { opt: ListOpt },

    /// Deletes a task by id
    Delete { id: usize },

    /// Toggles a task's completion state
    Toggle { id: usize },
}

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
#[value(rename_all = "lower")]
pub enum ListOpt {
    All,
    Done,
    Undone,
}
