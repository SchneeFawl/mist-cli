mod cli;
mod commands;
mod dependencies;

use clap::Parser;
use cli::{Cli};

use crate::commands::run;

fn main() {
    let cli = Cli::parse();

    match run(cli.command) {
        Ok(()) => (),
        Err(err) => eprintln!("Error parsing command: {err}\n")
    };
}
