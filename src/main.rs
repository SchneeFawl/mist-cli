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

    // let deps = &config.dependencies;
    // for dep in deps {
    //     let pkg: &str = &dep.name;
    //     match is_installed(&pkg) {
    //         Ok(status) => if status == true {
    //             println!("{} is installed", &pkg)
    //         } else {
    //             println!("{} is not installed", &pkg)
    //         },
    //         Err(error) => println!("Could not check installation status for {}: {error}", &pkg)
    //     }
    // }
}
