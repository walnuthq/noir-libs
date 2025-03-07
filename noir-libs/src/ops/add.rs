use std::{env, path::PathBuf};
use std::path::Path;
use crate::api::api::{download_package_api, get_latest_package_version_api};
use crate::tar::extract_tar_gz;
use crate::{
    config::MANIFEST_FILE_NAME,
    filesystem::prepare_cache_dir,
    manifest::{get_dependencies, write_package_dep},
    path::{get_cache_storage, get_package_dir},
};

pub fn add(package_name: &str, version: &str) -> Result<String, String> {
    let pwd = env::current_dir().expect("Unable to find current folder");
    let manifest_path = crate::manifest::try_find_manifest(&pwd).expect(format!("Unable to find {} manifest file", &MANIFEST_FILE_NAME).as_str());
    let manifest_dir = Path::new(&manifest_path)
        .parent()
        .expect("Failed to get manifest parent directory");
    let cache_root = prepare_cache_dir(&manifest_dir.to_path_buf());

    let used_version = get_used_version(package_name, version)?;

    store_package(cache_root.clone(), package_name, &used_version, false)?;
    add_dep_to_manifest(&manifest_path, manifest_dir.to_path_buf(), cache_root, package_name, &used_version);

    Ok(used_version)
}

pub fn store_package(
    cache_root: PathBuf,
    package_name: &str,
    version: &str,
    fetch_yanked: bool,
) -> Result<(), String> {
    // Get the package into the cache
    let package_cache_path = get_to_cache(cache_root.clone(), package_name, version, fetch_yanked)?;

    // Resolve sub-dependencies (if any)
    let package_manifest_path = package_cache_path.join(MANIFEST_FILE_NAME);
    //println!("Searching deps {:?}", package_manifest_path);
    if package_manifest_path.is_file() {
        let sub_deps = get_dependencies(package_manifest_path);

        for (sub_dep_name, sub_dep_version) in sub_deps {
            //      println!("Found dep {} {}", sub_dep_name, &sub_dep_version);
            store_package(
                cache_root.clone(),
                &sub_dep_name,
                &sub_dep_version,
                fetch_yanked,
            )?;
        }
    }
    Ok(())
}

fn add_dep_to_manifest(
    manifest_path: &PathBuf,
    manifest_dir: PathBuf,
    cache_root: PathBuf,
    package_name: &str,
    version: &str,
) {
    let cached_package_path = get_package_dir(cache_root, package_name, version);

    let relative_path = cached_package_path.strip_prefix(manifest_dir)
        .unwrap_or(&cached_package_path)
        .to_path_buf();

    write_package_dep(
        manifest_path,
        package_name,
        relative_path.to_str().unwrap(),
    );
}

/// Retrieves a package from the cache, downloading it if necessary,
/// and extracts it to the specified directory.
///
/// # Arguments
///
/// * `cache_root` - The root directory of the cache.
/// * `package_name` - The name of the package to retrieve.
/// * `version` - The version of the package to retrieve.
/// * `fetch_yanked` - fetch package even if yanked (used for fetch command)
///
/// # Returns
///
/// Returns the path to the cached package.
fn get_to_cache(cache_root: PathBuf, package_name: &str, version: &str,
            fetch_yanked: bool) -> Result<PathBuf, String> {
    let package_storage = get_cache_storage(cache_root.clone(), package_name, version);
    let cached_package_path = get_package_dir(cache_root, package_name, version);

    download_package_api(&package_storage, package_name, version, fetch_yanked)?;
    extract_tar_gz(&package_storage, &cached_package_path)
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
        get_latest_package_version_api(package_name)
    } else {
        Ok(version.to_string())
    }
}
