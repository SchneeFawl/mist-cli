use std::process::{ Command };

pub fn is_installed(name: &str) -> std::io::Result<bool> {
    let cmd = Command::new("pacman")
        .arg("-Q")
        .arg(name)
        .output()?;

    Ok(cmd.status.success())
}
