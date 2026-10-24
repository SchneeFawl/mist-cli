use std::{
    collections::HashSet, io::{self, IsTerminal, Write, stdin}
};

use crate::{
    commands::deps::DepsError::{AurPackage, InstalledPackages, InvalidCatalog, TomlParse, UnknownPackage, Installer},
    dependencies::{catalog::{CatalogError, DependencyConfig, PackageSource},
    installer::{InstalledPackagesError, InstallerError, install_planned_packages, is_installed}}
};

pub fn deps_check() -> Result<(), DepsError> {
    let config = DependencyConfig::load()?;
    let deps = &config.dependencies;
    let validate = config.validate()?;

    let installed_set = is_installed()?;

    println!("{}", validate);
    println!("Number of dependencies: {}\n", config.count());

    for dep in deps {
        if installed_set.contains(&dep.name) {
            println!("{} is installed", dep.name);
        } else {
            println!("{} is not installed", dep.name);
        }
    };
    Ok(())
}

pub fn deps_install(packages: &[String]) -> Result<(), DepsError> {
    let config = DependencyConfig::load()?;
    config.validate()?;

    check_packages(packages, &config)?;
    let installed_set = is_installed()?;
    let plan = plan_pacman_install(packages, &installed_set)?;
    if plan.is_empty() {
        println!("All requested packages are already installed");
        return Ok(())
    }

    println!("Total packages to install: {}", plan.len());
    for package in &plan {
        println!("  - {}", package)
    }
    println!();

    if confirm_install() {
        println!("Installation confirmed");
        install_planned_packages(&plan)?
    } else {
        println!("Installation cancelled")
    }

    Ok(())
}

pub fn plan_pacman_install(
    packages: &[String],
    installed: &HashSet<String>
) -> Result<Vec<String>, DepsError> {
    let mut plan: Vec<String> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();

    for package in packages {
        if !installed.contains(package) && seen.insert(package.clone()) {
            plan.push(package.clone());
        }
    }

    Ok(plan)
}

pub fn check_packages(packages: &[String], config: &DependencyConfig) -> Result<(), DepsError> {
    for package in packages {
        match config.find(package) {
            Some(dep) => {
                if dep.source == PackageSource::Aur {
                    return Err(DepsError::AurPackage(dep.name.clone()));
                }
            },
            None => return Err(DepsError::UnknownPackage(package.to_string()))
        }
    }

    Ok(())
}

pub fn confirm_install() -> bool {
    let stdin = stdin();

    if !stdin.is_terminal() {
        return false;
    }

    print!("> Proceed with installation? [y/N] ");
    if io::stdout().flush().is_err() {
        return false;
    }

    let mut input: String = String::new();
    match stdin.read_line(&mut input) {
        Ok(0) | Err(_) => false,        // EOF
        Ok(_) => confirm_install_decision(&input)
    }
}

pub fn confirm_install_decision(input: &str) -> bool {
    let trimmed = input.trim();
    trimmed.eq_ignore_ascii_case("y") || trimmed.eq_ignore_ascii_case("yes")
}

#[derive(Debug)]
pub enum DepsError {
    TomlParse(toml::de::Error),
    InvalidCatalog(CatalogError),
    UnknownPackage(String),
    AurPackage(String),
    InstalledPackages(InstalledPackagesError),
    Installer(InstallerError)
}

impl std::fmt::Display for DepsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TomlParse(err) => writeln!(f, "Failed to parse dependencies.toml\n {}", err),
            InvalidCatalog(err) => writeln!(f, "Dependency catalog validation failed\n {err}"),
            UnknownPackage(err) => write!(f, "Found unknown package '{}'", err),
            AurPackage(err) => write!(f, "Found AUR package: {}. AUR package installation isn't supported yet", err),
            InstalledPackages(err) => write!(f, "Failed to query installed packages\n {}", err),
            Installer(err) => write!(f, "Failed to install planned packages\n {}", err)
        }
    }
}

impl From<toml::de::Error> for DepsError {
    fn from(error: toml::de::Error) -> Self {
        DepsError::TomlParse(error)
    }
}

impl From<CatalogError> for DepsError {
    fn from(error: CatalogError) -> Self {
        DepsError::InvalidCatalog(error)
    }
}

impl From<InstalledPackagesError> for DepsError {
    fn from(error: InstalledPackagesError) -> Self {
        DepsError::InstalledPackages(error)
    }
}

impl From<InstallerError> for DepsError {
    fn from(error: InstallerError) -> Self {
        DepsError::Installer(error)
    }
}

#[cfg(test)]
mod test {
use super::*;

    #[test]
    fn invalid_toml_to_toml_parse_error() {
        let invalid_toml = r#"
            [[dependencies]
            name = "some_package"
            description = "Some package"
            source = "aur"
        "#;

        let parsed_result: Result<DependencyConfig, _> = toml::from_str(invalid_toml);
        assert!(parsed_result.is_err(), "Expected TOML parsing to fail");

        let deps_error: DepsError = parsed_result.unwrap_err().into();

        match &deps_error {
            DepsError::TomlParse(err) => {
                let displayed = deps_error.to_string();

                assert!(
                    displayed.contains("Failed to parse dependencies.toml"),
                    "Missing expected pattern in '{displayed}'"
                );
                assert!(
                    displayed.contains(&err.to_string())
                );
            },
            _ => panic!("Expected DepsError::TomlParse variant but got: {:?}", &deps_error)
        }
    }

    #[test]
    fn empty_dep_name_to_invalid_catalog_error() {
        let invalid_dep_toml = r#"
            [[dependencies]]
            name = ""
            description = "No name package"
            source = "pacman"
        "#;

        let config: DependencyConfig = toml::from_str(invalid_dep_toml)
            .expect("TOML parse should succeed");

        let validate = config.validate();
        assert!(validate.is_err(), "Expected validate() to fail");

        let deps_error: DepsError = validate.unwrap_err().into();

        match &deps_error {
            DepsError::InvalidCatalog(CatalogError::InvalidDepName) => {
                let displayed = deps_error.to_string();

                assert!(
                    displayed.contains("Dependency catalog validation failed"),
                    "Missing expected pattern in {displayed}"
                );
                assert!(
                    displayed.contains("Invalid dependency name"),
                    "Missing CatalogError display pattern in {displayed}"
                );
            },
            _ => panic!(
                "Expected DepsError::InvalidCatalog(CatalogError::InvalidDepName) but got {:?}", &deps_error
            )
        }
    }

    #[test]
    fn plan_pacman_install_deduplication_and_missing() {
        let mut installed: HashSet<String> = HashSet::new();
        installed.insert("hyprland".to_string());

        let requested = vec![
            "hyprland".to_string(),
            "kitty".to_string(),
            "kitty".to_string()
        ];

        let plan = plan_pacman_install(&requested, &installed)
            .expect("Expected planning to succeed");

        // deduplicates "kitty", "hyprland" is filtered
        assert_eq!(plan, vec!["kitty".to_string()]);

        // if all packages are installed
        installed.insert("kitty".to_string());
        let all_installed_plan = plan_pacman_install(&requested, &installed)
            .expect("Expected planning to succeed");
        assert!(all_installed_plan.is_empty())
    }

    #[test]
    fn confirm_install_acceptance() {
        let input_yes: &str = "yes";
        let input_y: &str = "y";
        let input_n: &str = "n";
        let input_no: &str = "no";
        let input_empty: &str = "";

        assert!(confirm_install_decision(input_yes), "Expected 'yes' to be accepted");
        assert!(confirm_install_decision(input_y), "Expected 'y' to be accepted");
        assert!(!confirm_install_decision(input_n), "Expected 'n' to be rejected");
        assert!(!confirm_install_decision(input_no), "Expected 'no' to be rejected");
        assert!(!confirm_install_decision(input_empty), "Expected empty input to be rejected");
    }
}
