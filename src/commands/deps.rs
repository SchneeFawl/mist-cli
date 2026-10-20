use std::collections::HashSet;

use crate::{
    commands::deps::{DepsError::{AurPackage, InstalledPackages, InvalidCatalog, TomlParse, UnknownPackage}},
    dependencies::{catalog::{CatalogError, DependencyConfig, PackageSource},
    installer::{InstalledPackagesError, is_installed}}
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
    deps_install_with_config(packages, &config)?;

    let installed_set = is_installed()?;
    let plan = plan_pacman_install(packages, &config, &installed_set)?;
    if plan.is_empty() {
        println!("All requested packages are already installed");
        Ok(())
    } else {
        for package in plan {
            println!("'{}' is not installed", package)
        }
        Ok(())
    }
}

pub fn plan_pacman_install(
    packages: &[String],
    config: &DependencyConfig,
    installed: &HashSet<String>
) -> Result<Vec<String>, DepsError> {
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

    let mut plan: Vec<String> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();

    for package in packages {
        if !installed.contains(package) && seen.insert(package.clone()) {
            plan.push(package.clone());
        }
    }

    Ok(plan)
}

fn deps_install_with_config(packages: &[String], config: &DependencyConfig) -> Result<(), DepsError> {
    let mut valid_pkgs: Vec<String> = Vec::new();

    for package in packages {
        match config.find(package) {
            Some(dep) => {
                if dep.source == PackageSource::Aur {
                    return Err(DepsError::AurPackage(dep.name.clone()))
                } else {
                    valid_pkgs.push(dep.name.clone());
                }
            },
            None => return Err(DepsError::UnknownPackage(package.to_string()))
        }
    }

    for valid_pkg in valid_pkgs {
        println!("Validated package '{}'", valid_pkg);
    }
    Ok(())
}

#[derive(Debug)]
pub enum DepsError {
    TomlParse(toml::de::Error),
    InvalidCatalog(CatalogError),
    InstalledPackages(InstalledPackagesError),
    UnknownPackage(String),
    AurPackage(String)
}

impl std::fmt::Display for DepsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TomlParse(err) => writeln!(f, "Failed to parse dependencies.toml\n {}", err),
            InvalidCatalog(err) => writeln!(f, "Dependency catalog validation failed\n {err}"),
            InstalledPackages(err) => write!(f, "Failed to query installed packages\n {}", err),
            UnknownPackage(err) => write!(f, "Found unknown package '{}'", err),
            AurPackage(err) => write!(f, "Found AUR package: {}. AUR package installation isn't supported yet", err)
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
    fn validate_pacman_packages() {
        let toml_string = r#"
            [[dependencies]]
            name = "hyprland"
            description = "Wayland compositor"
            source = "pacman"

            [[dependencies]]
            name = "quickshell-git"
            description = ""
            source = "aur"
        "#;

        let config: DependencyConfig = toml::from_str(toml_string)
            .expect("TOML parse should succeed");

        let pacman_pkg: &[String] = &["hyprland".to_string()];
        let aur_pkg: &[String] = &["quickshell-git".to_string()];
        let unknown_pkg: &[String] = &["something".to_string()];

        let pacman_result = deps_install_with_config(pacman_pkg, &config);
        assert!(!pacman_result.is_err(), "Expected pacman package to install");

        let unknown_err = deps_install_with_config(unknown_pkg, &config);
        assert!(matches!(unknown_err, Err(DepsError::UnknownPackage(_))));

        let aur_err = deps_install_with_config(aur_pkg, &config);
        assert!(matches!(aur_err, Err(DepsError::AurPackage(_))));
    }
}
