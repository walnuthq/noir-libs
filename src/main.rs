use clap::{Parser, Subcommand};
use noir_libs::{filesystem::{extract_package, prepare_cache_dir}, manifest::{try_find_manifest, write_package}, network::download_remote};
use pathdiff::diff_paths;

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
     println!("Adding package: {}", package_name);
    let dir = prepare_cache_dir();
    let path_with_version = dir.join(format!("{}-{}", package_name, version));
    let path_without_version = dir.join( package_name);
    download_remote(&path_with_version, package_name, version);
    extract_package(&path_with_version, &path_without_version, version).expect("Problem extracting package"); 



    let manifest_path = try_find_manifest().unwrap();
    let target_dir = dir.join("protocol_types").join(version);
    // Get the directory containing the manifest
    let project_dir = manifest_path.parent().expect("Failed to get project directory");

    // Compute the relative path to `target_dir` from the project directory
    let relative_path = diff_paths(&target_dir, project_dir)
        .expect("Failed to compute relative path")
        .to_string_lossy()
        .into_owned();

    write_package(manifest_path, package_name, &relative_path);
}

fn remove_package(package_name: &str) {
    println!("Removing package: {}", package_name);
    // TODO: Implement package removal logic
}
