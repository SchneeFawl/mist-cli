mod cli;
// mod commands;
mod dependencies;
use clap::Parser;
use cli::{Cli};

use crate::dependencies::catalog::{DependencyConfig, PackageSource};

fn main() {
    let _cli = Cli::parse();
    let config = DependencyConfig::load().expect("dependencies.toml is either invalid or corrupted");

    println!("{:?}", DependencyConfig::find(&config, &"hyprland"));
    config.by_source(PackageSource::Pacman)
        .for_each(|dep| println!("{:?}", dep));

    println!("{:?}", config.validate());
}
