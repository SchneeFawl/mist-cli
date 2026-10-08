mod cli;
mod commands;
mod dependencies;

use clap::Parser;
use cli::{Cli};

use crate::commands::run;
use crate::dependencies::catalog::{DependencyConfig, PackageSource};
use crate::dependencies::installer::{is_installed};

fn main() {
    let cli = Cli::parse();
    let config = DependencyConfig::load()
        .expect("dependencies.toml is either invalid or corrupted");

    match run(cli.command) {
        Ok(result) => println!("{result}\n"),
        Err(err) => println!("Error parsing command: {err}\n")
    };

    match config.find("hyprland") {
        Some(dep) => println!("{}", dep),
        None => println!("Dependency not found")
    };

    config.by_source(PackageSource::Aur)
        .for_each(|dep| println!("{}", dep));

    match config.validate() {
        Ok(success) => println!("{success}\n"),
        Err(error) => println!("{error}\n")
    }

    let deps = &config.dependencies;
    for dep in deps {
        let pkg: &str = &dep.name;
        match is_installed(&pkg) {
            Ok(status) => if status == true {
                println!("{} is installed", &pkg)
            } else {
                println!("{} is not installed", &pkg)
            },
            Err(error) => println!("Could not check installation status for {}: {error}", &pkg)
        }
    }
}
