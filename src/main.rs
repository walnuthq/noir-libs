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
        /// Package in the format package@version
        package: String,
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
        Commands::Add { package } => {
            let parts: Vec<&str> = package.split('@').collect();
            if parts.len() == 2 {
                add_package(parts[0], parts[1]);
            } else {
                eprintln!("Invalid package format. Use package@version.");
            }
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
