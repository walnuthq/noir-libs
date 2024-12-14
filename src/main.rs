use clap::{Parser, Subcommand};
use noir_libs::ops::{add, remove};

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
        /// Package version
        version: String
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
        Commands::Add { package_name , version} => {
            add_package(package_name, version);
        }
        Commands::Remove { package_name } => {
            remove_package(package_name);
        }
    }
}

fn add_package(package_name: &str, version: &str) {
     add(&package_name, &version);
}

fn remove_package(package_name: &str) {
    remove(&package_name);
}
