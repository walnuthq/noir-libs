use std::{
    fs::{self},
    io,
    path::{Path, PathBuf},
};

use crate::path::get_cache_dir;
use ignore::WalkBuilder;

pub fn prepare_cache_dir() -> PathBuf {
    let cache_dir = get_cache_dir().expect("Could not determine cache directory");
    ensure_dir(&cache_dir).expect("Failed to setup cache directory");
    cache_dir
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
        // copy hidden files (starting with .)
        .hidden(false)
        .filter_entry(move |e| {
            let path = e.path();

            // always pass .gitignore
            if path.file_name().map_or(false, |name| name == ".gitignore") {
                return true;
            }

            // ignore dest folder
            if path.starts_with(&dest_clone) {
                return false;
            }

            // ignore files from ignore_files list
            if ignore_files_clone.iter().any(|ig| path.ends_with(ig)) {
                return false;
            }

            // ignore folders from ignore_folders
            if let Some(folder_name) = path.file_name().and_then(|n| n.to_str()) {
                if ignore_folders_clone.contains(&folder_name.to_string()) {
                    return false;
                }
            }

            true
        })
        .build()
    {
        let entry = match entry {
            Ok(e) => e,
            Err(err) => {
                eprintln!("ERROR: {}", err);
                continue;
            }
        };

        println !("Processing file: {:?}", entry.path());

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
    use super::*;
    use std::fs;
    use tempfile::tempdir;

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
