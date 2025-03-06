use std::path::PathBuf;

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
        let cache_root = if cfg!(target_os = "windows") {
            PathBuf::from("C:\\Users\\user\\AppData\\Local\\noir-libs")
        } else {
            PathBuf::from("/home/user/.cache/noir-libs")
        };

        let result = get_cache_storage(cache_root.clone(), "value_note", "0.67.0");

        let expected_path = if cfg!(target_os = "windows") {
            "C:\\Users\\user\\AppData\\Local\\noir-libs\\value_note_0.67.0.archive"
        } else {
            "/home/user/.cache/noir-libs/value_note_0.67.0.archive"
        };

        assert_eq!(result.to_str().unwrap(), expected_path);
    }

    #[test]
    fn test_get_package_dir() {
        let cache_root = if cfg!(target_os = "windows") {
            PathBuf::from("C:\\Users\\user\\AppData\\Local\\noir-libs")
        } else {
            PathBuf::from("/home/user/.cache/noir-libs")
        };

        let result = get_package_dir(cache_root.clone(), "value_note", "0.67.0");

        let expected_path = if cfg!(target_os = "windows") {
            "C:\\Users\\user\\AppData\\Local\\noir-libs\\value_note\\0.67.0"
        } else {
            "/home/user/.cache/noir-libs/value_note/0.67.0"
        };

        assert_eq!(result.to_str().unwrap(), expected_path);
    }
}
