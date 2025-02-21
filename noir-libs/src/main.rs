use anyhow::bail;
use clap::{CommandFactory, Parser, Subcommand};
use colored::Colorize;
use noir_libs::config::PACKAGING_OUTPUT_FOLDER_PATH;
use noir_libs::ops::add::add;
use noir_libs::ops::package::package::package;
use noir_libs::ops::publish::publish;
use noir_libs::ops::remove;
use noir_libs::ops::yank::yank;

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
        /// Packages in the format "package@version" or "package" for the latest version
        packages: Vec<String>,
    },
    /// Removes packages from the project
    Remove {
        /// Names of the packages to remove
        package_names: Vec<String>,
    },

    /// Packages a local package into distributable tarball.
    Package {},

    /// Package and publish local package tarball to the remote registry.
    Publish {},

    /// Yank a package version (disable from being automatically downloaded). You must be owner of the package.
    Yank {
        /// Package to yank in the format "package@version"
        package: String
    }
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
                let (package_name, version) = split_package_to_name_and_version(package);
                add_package(package_name, version);
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
        Commands::Package {} => {
            let manifest_folder = std::env::current_dir().unwrap();
            let dst_folder = std::env::current_dir().unwrap().join(PACKAGING_OUTPUT_FOLDER_PATH);
            match package(&manifest_folder, &dst_folder) {
                Ok(packaged_tarball) => println!("{}", format!("Successfully packaged. Tarball path: {}", packaged_tarball.tarball_path).green().bold()),
                Err(e) => {
                    println!("{}", format!("Error: {}", e).red().bold());
                }
            }
            std::process::exit(1);
        }
        Commands::Publish {} => {
            match publish() {
                Ok(result_message) => println!("{}", result_message.green().bold()),
                Err(e) => {
                    println!("{}", format!("Error: {}", e).red().bold());
                }
            }
            std::process::exit(1);
        }
        Commands::Yank { package } => {
            match split_package_to_name_and_version_with_validation(package) {
                Ok((package_name, version)) => {
                    match yank(package_name, version) {
                        Ok(result_message) => println!("{}", result_message.green().bold()),
                        Err(e) => { println!("{}", format!("Error: {}", e).red().bold()); }
                    }
                },
                Err(e) => { println!("{}", format!("Error: {}", e).red().bold()); }
            }
            std::process::exit(1);
        }
    }
}

fn add_package(package_name: &str, version: &str) {
    match add(package_name, version) {
        Ok(ver) => println!(
            "Successfully installed package {}@{} and updated configuration!",
            package_name, ver
        ),
        Err(err) => {
            println!("Error: {}", err);
        }
    };
}

fn remove_package(package_name: &str) {
    remove::remove(package_name);
    println!("Successfully removed package {}", package_name);
}

fn split_package_to_name_and_version(package: &String) -> (&str, &str) {
    let parts: Vec<&str> = package.split('@').collect();
    let version = if parts.len() == 2 {
        parts[1]
    } else {
        "latest" // Use "latest" if no version is specified
    };
    (parts[0], version)
}

fn split_package_to_name_and_version_with_validation(package: &String) -> anyhow::Result<(&str, &str)> {
    let parts: Vec<&str> = package.split('@').collect();
    let version = if parts.len() == 2 {
        let version = parts[1];
        if let Err(e) = semver::Version::parse(version) {
            bail!("Package version {} is incorrect. Assure correct semantic versioning value.", version);
        }
        version
    } else {
        bail!("Please provide a package in a format <package-name>@<version>")
    };
    anyhow::Ok((parts[0], version))
}