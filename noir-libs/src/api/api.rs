use crate::api::network::{download_package, get_latest_package_version, publish_package};
use crate::config::REGISTRY_URL;
use crate::ops::package::PackagedTarball;
use std::path::Path;

pub fn download_package_api(output_path: &Path, package_name: &str, version: &str) -> Result<(), String> {
    download_package(&output_path, get_package_url(package_name, version).as_str())
}

pub fn get_latest_package_version_api(package_name: &str) -> Result<String, String> {
    get_latest_package_version(get_latest_package_version_url(package_name))
}

pub fn publish_package_api(tarball_path: &PackagedTarball) -> anyhow::Result<(String)> {
    publish_package(tarball_path, get_publish_package_url(tarball_path.name.as_str(), tarball_path.version.as_str()))
}

/// Retrieves the URL where to retrieve a package
/// Example: http://localhost:3001/api/v1/packages/aztec/0.67.0/download
fn get_package_url(package_name: &str, version: &str) -> String {
    format!("{}/packages/{}/{}/download", REGISTRY_URL, package_name, version)
}

/// Retrieves the URL for finding the latest version for a package
/// Example: http://localhost:3001/api/v1/packages/aztec/latest
fn get_latest_package_version_url(package_name: &str) -> String {
    format!("{}/packages/{}/versions/latest", REGISTRY_URL, package_name)
}

/// Retrieves the URL for publishing a package to remote registry
/// Example: http://localhost:3001/api/v1/packages/my_cool_package/0.1.0/publish
fn get_publish_package_url(package_name: &str, version: &str) -> String {
    format!("{}/packages/{}/{}/publish", REGISTRY_URL, &package_name, &version)
}

