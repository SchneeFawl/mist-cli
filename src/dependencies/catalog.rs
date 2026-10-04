use serde::{Deserialize};

#[derive(Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PackageSource {
    Pacman,
    Aur
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Dependency {
    pub name: String,
    pub description: String,
    pub source: PackageSource
}

const CATALOG: &str = include_str!("dependencies.toml");

#[derive(Deserialize, Debug)]
pub struct DependencyConfig {
    pub dependencies: Vec<Dependency>
}

impl DependencyConfig {
    pub fn load() -> Result<Self, toml::de::Error> {
        toml::from_str(CATALOG)
    }

    pub fn by_source(&self, source: PackageSource) -> impl Iterator<Item = &Dependency> {
        self.dependencies
            .iter()
            .filter(move |dep| dep.source == source)
    }

    pub fn find(&self, query: &str) -> Option<&Dependency> {
        self.dependencies
            .iter()
            .find(|dep| dep.name == query)
    }

    pub fn validate(&self) -> Result<(), CatalogError> {
        let filtered = self.dependencies
            .iter()
            .find(|dep| dep.name.is_empty());

        match filtered {
            Some(_invalid_dep) => return Err(
                CatalogError::InvalidDep("Invalid dependency name".to_string())
            ),
            None => return Ok(())
        }
    }
}

#[derive(Debug)]
pub enum CatalogError {
    InvalidDep(String)
}
