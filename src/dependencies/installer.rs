use std::process::{ Command };
use std::str::{Utf8Error, from_utf8};
use std::collections::HashSet;

use crate::dependencies::installer::InstalledPackagesError::{IoError, UnsuccessfulExit, Utf8Decode};

pub fn is_installed(name: &str) -> std::io::Result<bool> {
    let cmd = Command::new("pacman")
        .arg("-Qq")
        .arg(name)
        .output()?;

    Ok(cmd.status.success())
}

pub fn parse_installed_packages(bytes: &[u8]) -> Result<HashSet<String>, Utf8Error> {
    let text = from_utf8(bytes)?;

    let packages = text
        .lines()
        .filter_map(|line| line.split_whitespace().next())
        .map(String::from)
        .collect();

    Ok(packages)
}

pub fn is_installed_alt() -> Result<(), std::io::Error> {
    let cmd = Command::new("pacman")
        .arg("-Qq")
        .output()?;

    Ok(())
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum InstalledPackagesError {
    IoError(std::io::Error),
    UnsuccessfulExit(std::process::ExitStatus),
    Utf8Decode(std::str::Utf8Error)
}

impl std::fmt::Display for InstalledPackagesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IoError(e) => write!(f, "Failed to start pacman process:\n {}", e),
            UnsuccessfulExit(e) => write!(f, "Process exited with exit code {:?}", Some(e.code())),
            Utf8Decode(_) => write!(f, "stdout produced invalid UTF-8 bytes"),
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
}
