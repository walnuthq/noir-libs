use directories::ProjectDirs;
use std::path::PathBuf;

use crate::config::{COMPANY_NAME, COMPANY_TLD};

/// Gets a cache directory, based on operation system
/// Linux: /home/user/.cache/noir-libs/
/// macOS: /Users/user/Library/Application Support/com.walnut.noir-libs/
/// Windows: C:\Users\Alice\AppData\Roaming\walnut\noir-libs
pub fn get_cache_dir() -> Option<PathBuf> {
    ProjectDirs::from(COMPANY_TLD, COMPANY_NAME, env!("CARGO_PKG_NAME"))
        .map(|proj_dirs| proj_dirs.cache_dir().to_path_buf())
}

/// Creates a full package name from package name and version
/// Example: aztec-0.67.0
pub fn get_full_package_name(package_name: &str, version: &str) -> String {
    format!("{}_{}", package_name, version)
}

/// Retrieves the filename of the package
/// Example: value_note-0.67.0.archive
pub fn get_package_filename(package_name: &str, version: &str) -> String {
    let full_package_name = get_full_package_name(package_name, version);
    format!("{}.archive", full_package_name)
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

    #[test]
    fn test_get_full_package_name() {
        let result = get_full_package_name("aztec", "0.67.0");
        assert_eq!(result, "aztec_0.67.0");
    }

    #[test]
    fn test_get_package_filename() {
        let result = get_package_filename("value_note", "0.67.0");
        assert_eq!(result, "value_note_0.67.0.archive");
    }

    #[test]
    fn test_get_cache_storage() {
        let cache_root = PathBuf::from("/home/user/.cache/noir-libs");
        let result = get_cache_storage(cache_root.clone(), "value_note", "0.67.0");
        assert_eq!(result.to_str().unwrap(), "/home/user/.cache/noir-libs/value_note_0.67.0.archive");
    }

    #[test]
    fn test_get_package_dir() {
        let cache_root = PathBuf::from("/home/user/.cache/noir-libs");
        let result = get_package_dir(cache_root.clone(), "value_note", "0.67.0");
        assert_eq!(result.to_str().unwrap(), "/home/user/.cache/noir-libs/value_note/0.67.0");
    }
}
