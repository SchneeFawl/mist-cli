mod cli;
use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Install { source } => println!("Installing into: {:?}", source)
    }
}
