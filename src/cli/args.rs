use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Args,
}

#[derive(Subcommand)]
enum Args {
    Init,
}
