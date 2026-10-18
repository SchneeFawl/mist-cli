pub mod deps;

use crate::{cli::Commands::{Deps, Install}, commands::deps::{DepsError, deps_check, deps_install}};
use crate::{cli::DepsCommands};

pub fn run(command: crate::cli::Commands) -> Result<(), DepsError> {
    match command {
        Install { source: _ } => {
            println!("'install' command is not implemented");
        }
        Deps { command: _ } => {
            deps_handler()?;
        }
    }

    Ok(())
}
