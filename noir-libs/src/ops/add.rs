use std::{env, path::PathBuf};

use crate::{
    config::MANIFEST_FILE_NAME,
    filesystem::{extract_package, prepare_cache_dir},
    manifest::{get_dependencies, write_package_dep},
    network::{download_remote, get_latest_version},
    path::{get_cache_storage, get_package_dir, get_package_latest_url, get_package_url},
};

pub fn add(package_name: &str, version: &str) -> Result<String, String> {
    let cache_root = prepare_cache_dir();
    let pwd = env::current_dir().expect("Unable to find current folder");

    let used_version = get_used_version(package_name, version)?;

    store_package(cache_root.clone(), pwd.clone(), package_name, &used_version)?;
    add_dep_to_manifest(pwd, cache_root, package_name, &used_version);

    Ok(used_version)
}

fn store_package(
    cache_root: PathBuf,
    project_dir: PathBuf,
    package_name: &str,
    version: &str,
) -> Result<(), String> {
    // Get the package into the cache
    let package_cache_path = get_to_cache(cache_root.clone(), package_name, version)?;

    // Resolve sub-dependencies (if any)
    let package_manifest_path = package_cache_path.join(MANIFEST_FILE_NAME);
    //println!("Searching deps {:?}", package_manifest_path);
    if package_manifest_path.is_file() {
        let sub_deps = get_dependencies(package_manifest_path);

        for (sub_dep_name, sub_dep_version) in sub_deps {
            //      println!("Found dep {} {}", sub_dep_name, &sub_dep_version);
            store_package(
                cache_root.clone(),
                project_dir.clone(),
                &sub_dep_name,
                &sub_dep_version,
            )?;
        }
    }
    Ok(())
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
fn get_to_cache(cache_root: PathBuf, package_name: &str, version: &str) -> Result<PathBuf, String> {
    let package_storage = get_cache_storage(cache_root.clone(), package_name, version);
    let cached_package_path = get_package_dir(cache_root, package_name, version);

    let url = get_package_url(package_name, version);

    download_remote(&package_storage, &url)?;
    extract_package(&package_storage, &cached_package_path)
        .map_err(|_| "Problem extracting package".to_string())?;

    Ok(cached_package_path)
}

/// Retrieves the used version of a package.
///
/// This function checks if the requested version is "latest". If so, it fetches
/// the latest version available for the specified package. Otherwise, it returns
/// the provided version as is.
///
/// # Arguments
///
/// * `package_name` - The name of the package for which to retrieve the version.
/// * `version` - The version string, which can be "latest" or a specific version.
///
/// # Returns
///
/// Returns a `String` representing the version of the package.
fn get_used_version(package_name: &str, version: &str) -> Result<String, String> {
    if version == "latest" {
        let latest_version_url = get_package_latest_url(package_name);
        get_latest_version(latest_version_url)
    } else {
        Ok(version.to_string())
    }
}
