use clap::{CommandFactory, Parser, Subcommand};
use colored::Colorize;
use noir_libs::ops::add::add;
use noir_libs::ops::package::package;
use noir_libs::ops::publish::publish;
use noir_libs::ops::remove;

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

    /// Publishes a local package to the remote registry.
    Publish {}
}

fn main() {
    // if let Err(e) = package() {
    //     println!("{}", format!("Error: {}", e).red().bold());
    // }

            // match package() {
            //     Ok(packaged_tarball) => println!("{}", format!("Successfully packaged. Tarball path: {}", packaged_tarball.tarball_path).green().bold()),
            //     Err(e) => {
            //         println!("{}", format!("Error: {}", e).red().bold());
            //     }
            // }

    match publish() {
                    Ok(result_message) => println!("{}", result_message.green().bold()),
                    Err(e) => {
                        println!("{}", format!("Error: {}", e).red().bold());
                    }
                }

    // let cli = Cli::parse();

    // match &cli.command {
    //     Commands::Add { packages } => {
    //         if packages.is_empty() {
    //             Cli::command()
    //                 .find_subcommand_mut("add")
    //                 .unwrap()
    //                 .print_help()
    //                 .unwrap();
    //             std::process::exit(1);
    //         }
    //         for package in packages {
    //             let parts: Vec<&str> = package.split('@').collect();
    //             let version = if parts.len() == 2 {
    //                 parts[1]
    //             } else {
    //                 "latest" // Use "latest" if no version is specified
    //             };
    //             add_package(parts[0], version);
    //         }
    //     }
    //     Commands::Remove { package_names } => {
    //         if package_names.is_empty() {
    //             Cli::command()
    //                 .find_subcommand_mut("remove")
    //                 .unwrap()
    //                 .print_help()
    //                 .unwrap();
    //             std::process::exit(1);
    //         }
    //         for package_name in package_names {
    //             remove_package(package_name);
    //         }
    //     }
    //     Commands::Package {} => {
    //         match package() {
    //             Ok(packaged_tarball) => println!("{}", format!("Successfully packaged. Tarball path: {}", packaged_tarball.tarball_path).green().bold()),
    //             Err(e) => {
    //                 println!("{}", format!("Error: {}", e).red().bold());
    //             }
    //         }
    //         std::process::exit(1);
    //     }
    //     Commands::Publish {} => {
    //         match publish() {
    //             Ok(result_message) => println!("{}", result_message.green().bold()),
    //             Err(e) => {
    //                 println!("{}", format!("Error: {}", e).red().bold());
    //             }
    //         }
    //         std::process::exit(1);
    //     }
    // }
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
