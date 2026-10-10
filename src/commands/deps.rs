use crate::{
    commands::deps::DepsError::{InvalidCatalog, TomlParse},
    dependencies::{catalog::{CatalogError, DependencyConfig}, installer::is_installed}
};

pub fn deps_handler() -> Result<(), DepsError> {
    let config = match DependencyConfig::load() {
        Ok(cfg) => cfg,
        Err(err) => return Err(DepsError::TomlParse(err))
    };
    let deps = &config.dependencies;

    match config.validate() {
        Ok(success) => {
            println!("{success}");
            println!("Number of dependencies: {}\n", config.count());

            deps.iter().for_each(|dep| {
                match is_installed(&dep.name) {
                    Ok(true) => println!("{} is installed", &dep.name),
                    Ok(false) => println!("{} is not installed", &dep.name),
                    Err(err) => eprintln!("Could not check installation status of {}: {}", &dep.name, err)
                }
            });

            Ok(())
        },
        Err(error) => return Err(DepsError::InvalidCatalog(error))
    }
}

#[derive(Debug)]
pub enum DepsError {
    TomlParse(toml::de::Error),
    InvalidCatalog(CatalogError)
}

impl std::fmt::Display for DepsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TomlParse(err) => write!(f, "Failed to parse dependencies.toml:\n {}", err),
            InvalidCatalog(err) => write!(f, "Dependency catalog validation failed:\n {err}")
        }
    }
}
