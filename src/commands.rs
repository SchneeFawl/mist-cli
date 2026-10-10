pub mod deps;

use crate::{cli::Commands::{Deps, Install}, commands::deps::deps_handler, dependencies::catalog::CatalogError};

pub fn run(command: crate::cli::Commands) -> Result<(), CatalogError> {
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
