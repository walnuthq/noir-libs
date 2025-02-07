use directories::ProjectDirs;
use std::path::PathBuf;

use crate::config::{COMPANY_NAME, COMPANY_TLD, REGISTRY_URL};

/// Gets a cache directory, based on operation system
/// Linux: /home/user/.cache/noir-libs/
/// macOS: /Users/user/Library/Application Support/com.walnut.noir-libs/
/// Windows: C:\Users\Alice\AppData\Roaming\walnut\noir-libs
pub fn get_cache_dir() -> Option<PathBuf> {
    ProjectDirs::from(COMPANY_TLD, COMPANY_NAME, env!("CARGO_PKG_NAME"))
        .map(|proj_dirs| proj_dirs.cache_dir().to_path_buf())
}

/// Retrieves the filename of the package
/// Example: value_note-0.67.0.archive
pub fn get_package_filename(package_name: &str, version: &str) -> String {
    format!("{}-{}.archive", package_name, version)
}

/// Retrieves the filename of the package in cache
/// Example: /home/user/.cache/noir-libs/value_note-0.67.0.archive
pub fn get_cache_storage(cache_root: PathBuf, package_name: &str, version: &str) -> PathBuf {
    cache_root.join(get_package_filename(package_name, version))
}

/// Retrieves the dir where a package's contents are stored in cache
/// Example: /home/user/.cache/noir-libs/value_note/0.67.0
pub fn get_package_dir(cache_root: PathBuf, package_name: &str, version: &str) -> PathBuf {
    cache_root.join(package_name).join(version)
}

/// Retrieves the URL where to retrieve a package
/// Example: http://localhost:3001/api/v1/packages/aztec/0.67.0/download
pub fn get_package_url(package_name: &str, version: &str) -> String {
    format!("{}/packages/{}/{}/download", REGISTRY_URL, package_name, version)
}

/// Retrieves the URL for finding the latest version for a package
/// Example: http://localhost:3001/api/v1/packages/aztec/latest
pub fn get_package_latest_url(package_name: &str) -> String {
    format!("{}/packages/{}/latest", REGISTRY_URL, package_name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use directories::ProjectDirs;

    #[test]
    fn test_get_cache_dir() {
        let package_name = env!("CARGO_PKG_NAME");
        let company_name = "walnut";
        let company_tld = "dev";

        // Create a mock ProjectDirs instance
        let proj_dirs = ProjectDirs::from(company_tld, company_name, package_name).unwrap();
        let cache_dir = proj_dirs.cache_dir().to_path_buf();

        // Call the function and assert the result
        let result = get_cache_dir();
        assert!(result.is_some());
        assert_eq!(result.unwrap(), cache_dir);
    }
}
