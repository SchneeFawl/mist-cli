use crate::dependencies::{catalog::{CatalogError, DependencyConfig}, installer::is_installed};

pub fn deps_handler() -> Result<(), CatalogError> {
    let config = match DependencyConfig::load() {
        Ok(cfg) => cfg,
        Err(err) => {
            eprintln!("ERROR: dependencies.toml is either invalid or corrupted:\n {err}");
            std::process::exit(1)
        }
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
        Err(error) => Err(error)
    }
}
