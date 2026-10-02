mod cli;
mod commands;
mod components;
use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();
}
