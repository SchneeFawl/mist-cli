use crate::dependencies::catalog::DependencyConfig;

pub fn deps_handler() {
    let config = DependencyConfig::load()
        .expect("dependencies.toml is either corrupted or invalid");

    match config.validate() {
        Ok(success) => println!("{success}"),
        Err(error) => println!("{error}")
    }

    println!("Number of dependencies: {}", config.count());
}
