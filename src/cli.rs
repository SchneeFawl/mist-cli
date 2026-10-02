use crate::commands;

use std::path::PathBuf;
use clap::{Parser, Subcommand};

/// CLI for mist-shell (mist-cli)
///
/// Simple to use CLI tool for managing mist-shell

#[derive(Parser, Debug)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Interactively install mist-shell
    Install {
        #[arg(default_value = "~/.config")]
        source: PathBuf
    },
    // Install(commands::install)

    // Interactively uninstall mist-shell component(s)
    // Uninstall {},

    // Returns information about mist-shell
    // Status {}
}
