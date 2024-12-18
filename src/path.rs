use crate::{COMPANY_NAME, COMPANY_TLD, REPOSITORY_URL};
use directories::ProjectDirs;
use std::path::PathBuf;

/// Gets a cache directory, based on operation system
/// Linux: ~/.cache/noir-libs/
/// macOS: ~/Library/Caches/com.walnut.noir-libs/
/// Windows: C:\Users\Username\AppData\Local\walnut\noir-libs\Cache
pub fn get_cache_dir() -> Option<PathBuf> {
    ProjectDirs::from(COMPANY_TLD, COMPANY_NAME, env!("CARGO_PKG_NAME"))
        .map(|proj_dirs| proj_dirs.cache_dir().to_path_buf())
}

/// Retrieves the filename of the package
/// Example: value_note-0.67.0
pub fn get_package_filename(package_name: &str, version: &str) -> String {
    format!("{}-{}", package_name, version)
}

/// Retrieves the filename of the package in cache
/// Example: /home/user/.cache/noir-libs/value_note-0.67.0
pub fn get_cache_storage(cache_root: PathBuf, package_name: &str, version: &str) -> PathBuf {
    cache_root.join(get_package_filename(package_name, version))
}

/// Retrieves the dir where a package's contents are stored in cache'
/// Example: /home/user/.cache/noir-libs/value_note/0.67.0
pub fn get_package_dir(cache_root: PathBuf, package_name: &str, version: &str) -> PathBuf {
    cache_root.join(package_name).join(version)
}

/// Retrieves the URL where to retrieve a package
/// Example: http://127.0.0.1:8888/value_note/0.67.0/value_note-0.67.0
pub fn get_package_url(package_name: &str, version: &str) -> String {
    format!(
        "{}/{}/{}/{}",
        REPOSITORY_URL,
        package_name,
        version,
        get_package_filename(package_name, version)
    )
}
