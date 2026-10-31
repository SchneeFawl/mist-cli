use serde::{Deserialize};
use std::collections::HashSet;

const CATALOG: &str = include_str!("dependencies.toml");

#[derive(Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PackageSource {
    Pacman,
    Aur
}

impl std::fmt::Display for PackageSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PackageSource::Aur => write!(f, "AUR"),
            PackageSource::Pacman => write!(f, "pacman")
        }
    }
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Dependency {
    pub name: String,
    pub description: String,
    pub source: PackageSource
}

impl std::fmt::Display for Dependency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} [{}] - {}", self.name, self.source, self.description)
    }
}

#[derive(Deserialize, Debug)]
pub struct DependencyConfig {
    pub dependencies: Vec<Dependency>
}

#[allow(dead_code)]
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

    pub fn validate(&self) -> Result<String, CatalogError> {
        let filtered_name = self.dependencies
            .iter()
            .find(|dep| dep.name.trim().is_empty());

        if filtered_name.is_some() {
            return Err(CatalogError::InvalidDepName)
        }

        let filtered_desc = self.dependencies
            .iter()
            .find(|dep| dep.description.trim().is_empty());

        if let Some(dep) = filtered_desc {
            return Err(CatalogError::InvalidDepDesc(dep.name.clone()));
        };

        let mut seen: HashSet<&str> = HashSet::new();
        for dep in &self.dependencies {
            if !seen.insert(&dep.name) {
                return Err(CatalogError::DuplicatePackage(dep.name.clone()))
            }
        };

        Ok("Dependencies validated".to_string())
    }

    pub fn count(&self) -> usize {
        self.dependencies.iter().len()
    }
}

#[derive(Debug)]
pub enum CatalogError {
    InvalidDepName,
    InvalidDepDesc(String),
    DuplicatePackage(String)
}

impl std::fmt::Display for CatalogError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CatalogError::InvalidDepName => write!(f, "Invalid dependency name"),
            CatalogError::InvalidDepDesc(name) => write!(f, "Invalid description found for '{}'", name),
            CatalogError::DuplicatePackage(name) => write!(f, "Duplicate package '{}' found", name)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn whitespace_name_validation() {
        let dep = Dependency {
            name: "    ".to_string(),
            description: "Whitespace named dependency".to_string(),
            source: PackageSource::Aur
        };

        let dep_config = DependencyConfig {
            dependencies: vec![dep]
        };

        let err = dep_config.validate().unwrap_err();
        assert!(
            matches!(
                err,
                CatalogError::InvalidDepName
            ),
            "Expected InvalidDepName error"
        )
    }

    #[test]
    fn whitespace_description_validation() {
        let dep_name: String = String::from("invalid-dep");
        let dep: Dependency = Dependency {
            name: dep_name.clone(),
            description: "      ".to_string(),
            source: PackageSource::Aur
        };

        let dep_config: DependencyConfig = DependencyConfig {
            dependencies: vec![dep]
        };
        let err = dep_config.validate().unwrap_err();

        assert!(
            matches!(
                err,
                CatalogError::InvalidDepDesc(pkg_name) if pkg_name == dep_name
            ),
            "Expected InvalidDepDesc error"
        )
    }

    #[test]
    fn deps_validate_deduplication() {
        let dep_name: String = String::from("some_package");
        let dep_1: Dependency = Dependency {
            name: dep_name.clone(),
            description: "some_package pacman".to_string(),
            source: PackageSource::Pacman
        };
        let dep_2: Dependency = Dependency {
            name: dep_name.clone(),
            description: "some_package AUR".to_string(),
            source: PackageSource::Aur
        };

        let dep_config: DependencyConfig = DependencyConfig {
            dependencies: vec![dep_1, dep_2]
        };

        let err = dep_config.validate().unwrap_err();
        assert!(
            matches!(
                err,
                CatalogError::DuplicatePackage(pkg_name) if pkg_name == dep_name
            )
        )
    }
}
