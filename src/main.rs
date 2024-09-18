use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "cli")]
#[command(about = "A CLI tool with subcommands", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {}

fn main() {
    let cli = Cli::parse();

    match &cli.command {}
}
