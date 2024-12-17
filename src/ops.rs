use std::{env, path::PathBuf};

use crate::{
    filesystem::{extract_package, get_cache_dir, prepare_cache_dir},
    manifest::{get_dependencies, remove_package, write_package_dep},
    network::download_remote,
    MANIFEST_FILE_NAME,
};

pub fn add(package_name: &str, version: &str) {
    println!("Adding package: {}", package_name);

    let pwd = env::current_dir().expect("Unable to find current folder");
    let cache_dir = get_cache_dir().expect("Cache directory unavailable");

    store_package(&package_name, &version, pwd.clone(), cache_dir.clone());

    add_dep_to_manifest(pwd, cache_dir, package_name, version);
}

pub fn store_package(package_name: &str, version: &str, project_dir: PathBuf, cache_dir: PathBuf) {
    // Get the package into the cache
    let package_cache_path = get_to_cache(cache_dir.clone(), package_name, version);

    // Resolve sub-dependencies (if any)
    let package_manifest_path = package_cache_path.join(MANIFEST_FILE_NAME);
    println!("Searching deps {:?}", package_manifest_path);
    if package_manifest_path.is_file() {
        let sub_deps = get_dependencies(package_manifest_path);

        for (sub_dep_name, sub_dep_version) in sub_deps {
            println!("Found dep {} {}", sub_dep_name, &sub_dep_version);
            store_package(
                &sub_dep_name,
                &sub_dep_version,
                project_dir.clone(),
                cache_dir.clone(),
            );
        }
    }
}

fn add_dep_to_manifest(
    project_dir: PathBuf,
    cache_dir: PathBuf,
    package_name: &str,
    version: &str,
) -> PathBuf {
    // Construct the path to the cached dependency
    let cached_package_path = cache_dir.join(package_name).join(version);
    println!("Writing path {:?}", cached_package_path);
    // Write the dependency into the project's manifest
    let manifest_path = write_package_dep(
        project_dir,
        package_name,
        cached_package_path.to_str().unwrap(),
    );
    manifest_path
}

fn get_to_cache(cache_root: PathBuf, package_name: &str, version: &str) -> PathBuf {
    let path_with_version = cache_root.join(format!("{}-{}", package_name, version));
    let path_without_version = cache_root.join(package_name);

    download_remote(&path_with_version, package_name, version);
    extract_package(&path_with_version, &path_without_version, version)
        .expect("Problem extracting package")
}
pub fn remove(package_name: &str) {
    remove_package(package_name);
}
