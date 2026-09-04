use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Args,
}

#[derive(Subcommand)]
pub enum Args {
    Init,
}
