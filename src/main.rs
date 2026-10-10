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
        Err(err) => {
            eprintln!("ERROR:  {err}");
            std::process::exit(1)
        }
    };
}
