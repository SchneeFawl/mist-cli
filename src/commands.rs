pub mod deps;

use crate::{cli::Commands::{Deps, Install}, commands::deps::deps_handler};

pub fn run(command: crate::cli::Commands) -> Result<(), String> {
    match command {
        Install { source: _ } => {
            println!("'install' command is not implemented");
        }
        Deps { command: _ } => {
            deps_handler();
        }
    }

    Ok(())
}
