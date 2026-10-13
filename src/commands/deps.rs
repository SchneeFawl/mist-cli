use crate::{
    commands::deps::DepsError::{InvalidCatalog, InvalidPackages, TomlParse}, dependencies::{catalog::{CatalogError, DependencyConfig}, installer::is_installed}
};

pub fn deps_handler() -> Result<(), DepsError> {
    let config = DependencyConfig::load()?;
    let deps = &config.dependencies;
    let validate = config.validate()?;

    println!("{}", validate);
    println!("Number of dependencies: {}\n", config.count());

    let mut invalid_packages: Vec<(String, std::io::Error)> = Vec::new();

    for dep in deps {
        match is_installed(&dep.name) {
            Ok(true) => println!("{} is installed", &dep.name),
            Ok(false) => println!("{} is not installed", &dep.name),
            Err(e) => {
                invalid_packages.push((dep.name.clone(), e));
            }
        }
    };

    if !invalid_packages.is_empty() {
        return Err(DepsError::InvalidPackages(invalid_packages));
    } else {
        Ok(())
    }
}

#[derive(Debug)]
pub enum DepsError {
    TomlParse(toml::de::Error),
    InvalidCatalog(CatalogError),
    InvalidPackages(Vec<(String, std::io::Error)>)
}

impl std::fmt::Display for DepsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TomlParse(err) => writeln!(f, "Failed to parse dependencies.toml:\n {}", err),
            InvalidCatalog(err) => writeln!(f, "Dependency catalog validation failed:\n {err}"),
            InvalidPackages(errors) => {
                writeln!(f, "Failed to check packages:")?;
                for (pkg_name, err) in errors {
                    writeln!(f, "  - {pkg_name}: {err}")?;
                }
                Ok(())
            }
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

#[cfg(test)]
mod test {
    use std::io::{Error, ErrorKind};
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
                    displayed.contains("Failed to parse dependencies.toml:"),
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
                    displayed.contains("Dependency catalog validation failed:"),
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
    fn two_io_errors_to_invalid_packages_error() {
        let err1: std::io::Error = Error::new(ErrorKind::NotFound, "pacman binary not found");
        let err2: std::io::Error = Error::new(ErrorKind::PermissionDenied, "access denied");
        let pkg1: String = String::from("hyprland");
        let pkg2: String = String::from("kitty");

        let vec: Vec<(String, Error)> = vec![
            (pkg1, err1),
            (pkg2, err2)
        ];

        let error = DepsError::InvalidPackages(vec);
        let displayed = error.to_string();

        assert!(
            displayed.contains("hyprland") && displayed.contains("binary not found"),
            "First package doesn't exist or its error in: \n{displayed}"
        );
        assert!(
            displayed.contains("kitty") && displayed.contains("access denied"),
            "Second package doesn't exist or its error in: \n{displayed}"
        );
    }
}
