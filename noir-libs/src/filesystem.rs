use std::{
    fs::{self, File},
    io,
    path::{Path, PathBuf},
};

use flate2::read::GzDecoder;
use tar::Archive;

use crate::path::get_cache_dir;
use ignore::{Walk, WalkBuilder};

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

pub fn new_dir_replace_if_exists(path: &Path) -> std::io::Result<()> {
    if path.exists() {
        fs::remove_dir_all(path)?;
    }
    fs::create_dir_all(path)?;
    Ok(())
}

pub fn copy_all(
    src: &Path,
    dest: &Path,
    ignore_folders: &[&str],
    ignore_files: &[&str],
) -> io::Result<()> {
    let dest_clone = dest.to_path_buf().clone();
    let ignore_files_clone: Vec<String> = ignore_files.iter().map(|s| s.to_string()).collect();
    let ignore_folders_clone: Vec<String> = ignore_folders.iter().map(|s| s.to_string()).collect();
    for entry in WalkBuilder::new(src)
        // ignore dest path
        .filter_entry(move |e| !e.path().starts_with(&dest_clone))
        // ignore files in a ignore_files list
        .filter_entry(move |e| ignore_files_clone.iter().all(|ig| !e.path().ends_with(ig)))
        // ignore folders in a ignore_folders list
        .filter_entry(move |e| ignore_folders_clone.iter().all(|ig| !e.path().to_string_lossy().contains(&format!("/{}/", ig))))
        .build()
    {
        let entry = match entry {
            Ok(e) => e,
            Err(err) => {
                eprintln!("ERROR: {}", err);
                continue;
            }
        };

        println!("Processing file: {:?}", entry.path());

        let path = entry.path();

        let relative_path = path.strip_prefix(src).unwrap_or(path);
        let dest_path = dest.join(relative_path);

        if path.is_dir() {
            fs::create_dir_all(&dest_path)?;
        } else if path.is_file() {
            fs::copy(&path, &dest_path)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::config::MANIFEST_FILE_NAME;

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
