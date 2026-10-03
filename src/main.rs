mod cli;
// mod commands;
mod dependencies;
use clap::Parser;
use cli::{Cli};

use crate::dependencies::catalog::{DependencyConfig};

fn main() {
    let _cli = Cli::parse();

    println!("{:?}", DependencyConfig::load_file());
}
