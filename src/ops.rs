use pathdiff::diff_paths;

use crate::{filesystem::{extract_package, prepare_cache_dir}, manifest::{remove_package, try_find_manifest, write_package}, network::download_remote};

pub fn add(package_name: &str, version: &str) {
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

pub fn remove(package_name: &str) {
    let manifest_path = try_find_manifest().unwrap();
    remove_package(manifest_path, &package_name);
}