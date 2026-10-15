use std::process::{ Command };
use std::str::{Utf8Error, from_utf8};
use std::collections::HashSet;

use crate::dependencies::installer::InstalledPackagesError::{Io, UnsuccessfulExit, Utf8Decode};

pub fn is_installed(name: &str) -> std::io::Result<bool> {
    let cmd = Command::new("pacman")
        .arg("-Qq")
        .arg(name)
        .output()?;

    Ok(cmd.status.success())
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

pub fn is_installed_alt() -> Result<HashSet<String>, InstalledPackagesError> {
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
    success: bool, exit_code: Option<i32>, stdout: &[u8]
) -> Result<HashSet<String>, InstalledPackagesError> {
    if !success {
        return Err(InstalledPackagesError::UnsuccessfulExit(exit_code))
    }
    parse_installed_packages(stdout)
}

#[derive(Debug)]
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

    // #[test]
    // fn unsuccessful_status() {
    //     let output = Command::new("false").output()?;
    //     let result = process_listing(
    //         output.status.success(), output.status.code(), &output.stdout
    //     );
    // }
}
