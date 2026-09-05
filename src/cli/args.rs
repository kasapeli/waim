use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Args,
}

#[derive(Subcommand)]
pub enum Args {
    Init,
    Add { task: String },
    List { opt: ListOpt },
}

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
#[value(rename_all = "lower")]
pub enum ListOpt {
    All,
    Done,
    Undone,
}
