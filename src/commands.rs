pub mod deps;

use crate::{cli::Commands::{Deps, Install}, commands::deps::{DepsError, deps_handler}};

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
