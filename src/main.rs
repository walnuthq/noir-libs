use clap::{Parser, Subcommand};

/// A CLI package manager for Noir
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds a package to the project
    Add {
        /// Name of the package to add
        package_name: String,
    },
    /// Removes a package from the project
    Remove {
        /// Name of the package to remove
        package_name: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { package_name } => {
            add_package(package_name);
        }
        Commands::Remove { package_name } => {
            remove_package(package_name);
        }
    }
}

fn add_package(package_name: &str) {
    println!("Adding package: {}", package_name);
    // TODO: Implement package addition logic
}

fn remove_package(package_name: &str) {
    println!("Removing package: {}", package_name);
    // TODO: Implement package removal logic
}
