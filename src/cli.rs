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

    /// Dependency commands
    Deps {
        #[command(subcommand)]
        command: DepsCommands,
    },

    // Interactively uninstall mist-shell component(s)
    // Uninstall {},

    // Returns information about mist-shell
    // Status {}
}

#[derive(Subcommand, PartialEq, Debug)]
pub enum DepsCommands {
    Check,
    Install {
        #[arg(required = false)]
        packages: Vec<String>
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn deps_install_parse_with_empty_pkg_list() {
        let args = ["mist-cli", "deps", "install"];
        let parsed = Cli::try_parse_from(args)
            .expect("Failed to parse CLI arguments");

        assert!(
            matches!(
                parsed.command,
                Commands::Deps { command: arg } if arg == DepsCommands::Install { packages: vec![] }
            )
        )
    }

    #[test]
    fn deps_install_parse_with_non_empty_pkg_list() {
        let args = ["mist-cli", "deps", "install", "hyprland", "kitty"];
        let parsed = Cli::try_parse_from(args)
            .expect("Failed to parse CLI arguments");

        assert!(
            matches!(
                parsed.command,
                Commands::Deps { command: arg } if {
                    arg == DepsCommands::Install { packages: vec![
                        "hyprland".to_string(), "kitty".to_string()
                    ] }
                }
            )
        )
    }
}
