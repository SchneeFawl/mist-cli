mod cli;
// mod commands;
mod dependencies;
use clap::Parser;
use cli::{Cli};

use crate::dependencies::catalog::{DependencyConfig, PackageSource};

fn main() {
    let _cli = Cli::parse();
    let config = DependencyConfig::load()
        .expect("dependencies.toml is either invalid or corrupted");

    match config.find("hyprland") {
        Some(dep) => println!("{}", dep),
        None => println!("Dependency not found")
    };

    config.by_source(PackageSource::Aur)
        .for_each(|dep| println!("{}", dep));

    match config.validate() {
        Ok(success) => println!("{success}"),
        Err(error) => println!("{:?}", error)
    }
}
