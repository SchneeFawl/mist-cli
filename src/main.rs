use clap::Parser;

/// CLI for mist-shell (mist-cli)
///
/// Simple to use CLI tool for managing mist-shell

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    option: String
}

fn main() {
    let _args = Args::parse();
    println!("mist-cli");
}
