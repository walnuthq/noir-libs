use std::{
    fs::{self, File},
    io,
    path::{Path, PathBuf},
};

use flate2::read::GzDecoder;
use tar::Archive;

use crate::path::get_cache_dir;

pub fn prepare_cache_dir() -> PathBuf {
    let cache_dir = get_cache_dir().expect("Could not determine cache directory");
    ensure_dir(&cache_dir).expect("Failed to setup cache directory");
    cache_dir
}

/// Extracts a package from a tar.gz file.
///
/// # Parameters
/// - `path_with_version`: The path to the tar.gz file containing the package.
/// - `path_without_version`: The base path where the package should be extracted.
/// - `version`: The version of the package being extracted.
///
/// # Returns
/// Returns the path to the directory where the package was extracted, or an error if the extraction fails.
pub fn extract_package(package_path: &Path, extract_dir: &Path) -> io::Result<()> {
    // Open the tar.gz file
    let tar_gz = File::open(package_path)?;
    let gz = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(gz);

    archive.unpack(extract_dir)?;

    Ok(())
}

/// Ensures the cache directory exists
fn ensure_dir(path: &Path) -> std::io::Result<()> {
    if path.exists() {
        if path.is_file() {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "A file with the same name as the directory exists",
            ));
        }
    } else {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::MANIFEST_FILE_NAME;

    use super::*;
    use std::fs;
    use tempfile::tempdir;

    const TEST_PACKAGE: &str = "tests/test_files/test_package-1.2.3";

    #[test]
    fn test_extract_package() {
        let temp_dir = tempdir().unwrap();
        let package_path = Path::new(TEST_PACKAGE);

        let result = extract_package(package_path, temp_dir.path());
        assert!(result.is_ok());
        assert!(temp_dir.path().join(MANIFEST_FILE_NAME).exists()); // Extracted files should include manifest
    }

    #[test]
    fn test_ensure_dir_creates() {
        let temp_dir = tempdir().unwrap();

        let result = ensure_dir(temp_dir.path());
        assert!(result.is_ok());
        assert!(temp_dir.path().exists() && temp_dir.path().is_dir());
    }

    #[test]
    fn test_ensure_dir_file_exists() {
        let temp_dir = tempdir().unwrap();
        let temp_file = temp_dir.path().join("test_file.txt");

        fs::write(&temp_file, "test").unwrap();
        let result = ensure_dir(&temp_file);
        assert!(result.is_err());
        if let Err(e) = result {
            assert_eq!(
                e.to_string(),
                "A file with the same name as the directory exists"
            );
        }
    }

    #[test]
    fn test_ensure_dir_already_exists() {
        let temp_dir = tempdir().unwrap();

        let result = ensure_dir(temp_dir.path());
        assert!(result.is_ok());

        let result = ensure_dir(temp_dir.path());
        assert!(result.is_ok());
    }
}
