// pub mod install;

use crate::cli::Commands::{Deps, Install};

pub fn run(command: crate::cli::Commands) -> Result<String, String> {
    let cmd_result = match command {
        Install { source: _ } => String::from("'install' command is not implemented"),
        Deps { command: _ } => String::from("'deps' command is not implemented")
    };

    Ok(cmd_result)
}
