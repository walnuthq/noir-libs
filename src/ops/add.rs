use std::{env, path::PathBuf};

use crate::{
    filesystem::{extract_package, prepare_cache_dir},
    manifest::{get_dependencies, write_package_dep},
    network::download_remote,
    path::{get_cache_storage, get_package_dir, get_package_url},
    MANIFEST_FILE_NAME,
};

pub fn add(package_name: &str, version: &str) {
    let cache_root = prepare_cache_dir();
    let pwd = env::current_dir().expect("Unable to find current folder");

    store_package(cache_root.clone(), pwd.clone(), package_name, version);

    add_dep_to_manifest(pwd, cache_root, package_name, version);
}

fn store_package(cache_root: PathBuf, project_dir: PathBuf, package_name: &str, version: &str) {
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

/// Retrieves a package from the cache, downloading it if necessary,
/// and extracts it to the specified directory.
///
/// # Arguments
///
/// * `cache_root` - The root directory of the cache.
/// * `package_name` - The name of the package to retrieve.
/// * `version` - The version of the package to retrieve.
///
/// # Returns
///
/// Returns the path to the cached package.
fn get_to_cache(cache_root: PathBuf, package_name: &str, version: &str) -> PathBuf {
    let package_storage = get_cache_storage(cache_root.clone(), package_name, version); //cache_root.join(format!("{}-{}", package_name, version));
    let cached_package_path = get_package_dir(cache_root, package_name, version);

    let url = get_package_url(package_name, version);
    download_remote(&package_storage, &url);
    extract_package(&package_storage, &cached_package_path).expect("Problem extracting package");

    cached_package_path
}
