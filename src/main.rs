use clap::{CommandFactory, Parser, Subcommand};
use noir_libs::ops::{add, remove};

/// A CLI package manager for Noir | noir-libs.org
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds packages to the project
    Add {
        /// Packages in the format package@version
        packages: Vec<String>,
    },
    /// Removes packages from the project
    Remove {
        /// Names of the packages to remove
        package_names: Vec<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { packages } => {
            if packages.is_empty() {
                Cli::command()
                    .find_subcommand_mut("add")
                    .unwrap()
                    .print_help()
                    .unwrap();
                std::process::exit(1);
            }
            for package in packages {
                let parts: Vec<&str> = package.split('@').collect();
                if parts.len() == 2 {
                    add_package(parts[0], parts[1]);
                } else {
                    eprintln!(
                        "Invalid package format for '{}'. Use package@version.",
                        package
                    );
                    Cli::command()
                        .find_subcommand_mut("add")
                        .unwrap()
                        .print_help()
                        .unwrap();
                    std::process::exit(1);
                }
            }
        }
        Commands::Remove { package_names } => {
            if package_names.is_empty() {
                Cli::command()
                    .find_subcommand_mut("remove")
                    .unwrap()
                    .print_help()
                    .unwrap();
                std::process::exit(1);
            }
            for package_name in package_names {
                remove_package(package_name);
            }
        }
    }
}

fn add_package(package_name: &str, version: &str) {
    crate::add::add(package_name, version);
}

fn remove_package(package_name: &str) {
    crate::remove::remove(package_name);
}
