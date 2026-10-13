use std::process::{ Command };
use std::str::{Utf8Error, from_utf8};
use std::collections::HashSet;

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn complete_name_is_present() {
        let bytes: &[u8] = b"hyprland \nkitty \nzsh";

        let packages_set = parse_installed_packages(bytes)
            .expect("Failed to parse packages list bytes");

        assert!(packages_set.contains("hyprland"),
            "Missing expected dependency 'hyprland' in {:?}", &packages_set
        );
        assert!(packages_set.contains("kitty"),
            "Missing expected dependency 'kitty' in {:?}", &packages_set
        );
        assert!(packages_set.contains("zsh"),
            "Missing expected dependency 'zsh' in {:?}", &packages_set
        );

        assert!(!packages_set.contains("hypr"),
            "Expected pattern match to fail for 'hypr'"
        );
        assert!(!packages_set.contains("kit"),
            "Expected pattern match to fail for 'kit'"
        );
        assert!(!packages_set.contains("z"),
            "Expected pattern match to fail for 'z'"
        );
    }
}
