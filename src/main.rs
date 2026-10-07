mod cli;
mod dependencies;

use clap::Parser;
use cli::{Cli};

use crate::dependencies::catalog::{DependencyConfig, PackageSource};
use crate::dependencies::installer::{is_installed};

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
        Err(error) => println!("{error}")
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
