use std::process::{ Command };
use std::str::{Utf8Error, from_utf8};
use std::collections::HashSet;

use crate::dependencies::installer::InstalledPackagesError::{Io, UnsuccessfulExit, Utf8Decode};

pub fn is_installed() -> Result<HashSet<String>, InstalledPackagesError> {
    let output = Command::new("pacman")
        .arg("-Qq")
        .output()
        .map_err(InstalledPackagesError::Io)?;      // pacman spawn failure

    if !output.status.success() {
        return Err(InstalledPackagesError::UnsuccessfulExit(output.status.code()));
    }

    process_listing(output.status.success(), output.status.code(), &output.stdout)
}

pub fn process_listing(
    success: bool,
    exit_code: Option<i32>,
    stdout: &[u8]
) -> Result<HashSet<String>, InstalledPackagesError> {
    if !success {
        return Err(InstalledPackagesError::UnsuccessfulExit(exit_code))
    }
    parse_installed_packages(stdout)
}

pub fn parse_installed_packages(bytes: &[u8]) -> Result<HashSet<String>, InstalledPackagesError> {
    let text = from_utf8(bytes)?;

    let packages = text
        .lines()
        .filter_map(|line| line.split_whitespace().next())
        .map(String::from)
        .collect();

    Ok(packages)
}

pub fn install_planned_pacman_packages(packages: &[String]) -> Result<(), InstallerError> {
    let mut cmd: Command = Command::new("sudo");
    cmd.args(["pacman", "-S", "--needed"]);
    cmd.args(packages);

    execute_install_command(cmd)
}

pub fn install_planned_aur_packages(packages: &[String]) -> Result<(), InstallerError> {
    let mut cmd: Command = Command::new("yay");
    cmd.args(["-S", "--needed"]);
    cmd.args(packages);

    execute_install_command(cmd)
}

pub fn execute_install_command(mut cmd: Command) -> Result<(), InstallerError> {
    let status = cmd
        .status()
        .map_err(|err| InstallerError::StartingProcess(
            cmd.get_program().to_string_lossy().into_owned(),
            err
        ))?;

    if !status.success() {
        return Err(InstallerError::UnsuccessfulExit(
            cmd.get_program().to_string_lossy().into_owned(),
            status.code()
        ))
    }
    Ok(())
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum InstalledPackagesError {
    Io(std::io::Error),
    UnsuccessfulExit(Option<i32>),
    Utf8Decode(std::str::Utf8Error)
}

impl std::fmt::Display for InstalledPackagesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Io(e) => write!(f, "Failed to start pacman process:\n {}", e),
            UnsuccessfulExit(e) => write!(f, "Process exited with exit code {:?}", e),
            Utf8Decode(_) => write!(f, "stdout produced invalid UTF-8 bytes"),
        }
    }
}

impl From<Utf8Error> for InstalledPackagesError {
    fn from(error: Utf8Error) -> Self {
        InstalledPackagesError::Utf8Decode(error)
    }
}

#[derive(Debug)]
pub enum InstallerError {
    StartingProcess(String, std::io::Error),
    UnsuccessfulExit(String, Option<i32>)
}

impl std::fmt::Display for InstallerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstallerError::StartingProcess(program, e) => {
                write!(f, "{} process failed to start:\n {}", program, e)
            },
            InstallerError::UnsuccessfulExit(program, Some(code)) => {
                write!(f, "{} process exited with exit code {}", program, code)
            },
            InstallerError::UnsuccessfulExit(program, None) => {
                write!(f, "{} process terminated by a signal (no exit code)", program)
            }
        }
    }
}

#[cfg(test)]
mod test {

use super::*;

    #[test]
    fn complete_name_is_present() {
        let bytes: &[u8] = b"hyprland \nkitty \nzsh";

        let packages_set = parse_installed_packages(bytes)
            .expect("Failed to parse packages list bytes");

        assert!(
            packages_set.contains("hyprland"),
            "Missing expected dependency 'hyprland' in {:?}", &packages_set
        );
        assert!(
            packages_set.contains("kitty"),
            "Missing expected dependency 'kitty' in {:?}", &packages_set
        );
        assert!(
            packages_set.contains("zsh"),
            "Missing expected dependency 'zsh' in {:?}", &packages_set
        );

        assert!(
            !packages_set.contains("hypr"),
            "Expected pattern match to fail for 'hypr'"
        );
        assert!(
            !packages_set.contains("kit"),
            "Expected pattern match to fail for 'kit'"
        );
        assert!(
            !packages_set.contains("z"),
            "Expected pattern match to fail for 'z'"
        );
    }

    #[test]
    fn reject_invalid_utf_8_bytes() {
        let invalid_bytes: &[u8] = &[0xFF, 167, 101, 0xFE];
        let invalid_packages = parse_installed_packages(invalid_bytes);

        assert!(
            invalid_packages.is_err(),
            "Expected parsing to fail"
        );
    }

    #[test]
    fn unsuccessful_status() {
        // if child status is unsuccessful
        let failed_result = process_listing(false, Some(1), b"hyprland\nkitty\n");
        assert!(
            matches!(
                failed_result,
                Err(InstalledPackagesError::UnsuccessfulExit(Some(1)))
            ),
            "Expected UnsuccessfulExit(Some(1)), but got: {:?}", failed_result
        );

        // successful listing excludes a name out of the set
        let set = b"hyprland\nkitty\n";
        let success_set = process_listing(true, Some(0), set)
            .expect("Successful status with valid UTF-8 should return a set");

        assert!(success_set.contains("hyprland"));
        assert!(success_set.contains("kitty"));
        assert!(
            !success_set.contains("waybar"),
            "'waybar' should not be in the set"
        );
    }

    #[test]
    fn install_planned_packages_command() {
        let cmd_success = Command::new("true");
        assert!(
            !execute_install_command(cmd_success).is_err(),
            "Expected commmand to run successfully"
        );

        let cmd_fail = Command::new("false");
        assert!(
            matches!(
                execute_install_command(cmd_fail),
                Err(InstallerError::UnsuccessfulExit(program, code)) if {
                    program == "false" &&
                    code == Some(1)
                }
            ),
            "Expected command to fail"
        );

        let cmd_nonexistent = Command::new("nonexistent-command");
        assert!(
            matches!(
                execute_install_command(cmd_nonexistent),
                Err(InstallerError::StartingProcess(program, _)) if {
                    program == "nonexistent-command"
                }
            ),
            "Expected command to fail"
        );
    }
}
