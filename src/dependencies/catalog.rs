use serde::Deserialize;

#[derive(Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PackageSource {
    Pacman,
    Aur
}

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
    pub fn load_file() -> Result<Self, toml::de::Error> {
        toml::from_str(CATALOG)
    }

    pub fn by_source(&self, source: PackageSource) -> impl Iterator<Item = &Dependency> {
        self.dependencies.iter()
            .filter(move |dep| dep.source == source)
    }
}
