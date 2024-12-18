use std::{env, path::PathBuf};

use crate::{
    filesystem::{extract_package, prepare_cache_dir},
    manifest::{get_dependencies, remove_package, write_package_dep},
    network::download_remote,
    path::{get_cache_storage, get_package_dir},
    MANIFEST_FILE_NAME,
};

pub fn add(package_name: &str, version: &str) {
    println!("Adding package: {}", package_name);

    let cache_root = prepare_cache_dir();
    let pwd = env::current_dir().expect("Unable to find current folder");

    store_package(cache_root.clone(), pwd.clone(), package_name, version);

    add_dep_to_manifest(pwd, cache_root, package_name, version);
}

pub fn store_package(cache_root: PathBuf, project_dir: PathBuf, package_name: &str, version: &str) {
    // Get the package into the cache
    let package_cache_path = get_to_cache(cache_root.clone(), package_name, version);

    // Resolve sub-dependencies (if any)
    let package_manifest_path = package_cache_path.join(MANIFEST_FILE_NAME);
    //println!("Searching deps {:?}", package_manifest_path);
    if package_manifest_path.is_file() {
        let sub_deps = get_dependencies(package_manifest_path);

        for (sub_dep_name, sub_dep_version) in sub_deps {
            //      println!("Found dep {} {}", sub_dep_name, &sub_dep_version);
            store_package(
                project_dir.clone(),
                cache_root.clone(),
                &sub_dep_name,
                &sub_dep_version,
            );
        }
    }
}

fn add_dep_to_manifest(
    project_dir: PathBuf,
    cache_root: PathBuf,
    package_name: &str,
    version: &str,
) -> PathBuf {
    // Construct the path to the cached dependency
    let cached_package_path = get_package_dir(cache_root, package_name, version);
    // Write the dependency into the project's manifest
    let manifest_path = write_package_dep(
        project_dir,
        package_name,
        cached_package_path.to_str().unwrap(),
    );
    manifest_path
}

fn get_to_cache(cache_root: PathBuf, package_name: &str, version: &str) -> PathBuf {
    let package_storage = get_cache_storage(cache_root.clone(), package_name, version); //cache_root.join(format!("{}-{}", package_name, version));
    let cached_package_path = get_package_dir(cache_root, package_name, version);

    download_remote(&package_storage, package_name, version);
    extract_package(&package_storage, &cached_package_path).expect("Problem extracting package");

    cached_package_path
}

pub fn remove(package_name: &str) {
    remove_package(package_name);
}
