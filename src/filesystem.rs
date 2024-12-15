use std::{fs::{self, File}, io, path::{Path, PathBuf}};

use directories::ProjectDirs;
use flate2::read::GzDecoder;
use tar::Archive;

use crate::version::VersionInfo;

/// Gets a cache directory, based on operation system
/// Linux: ~/.cache/noir-libs/
/// macOS: ~/Library/Caches/com.walnut.noir-libs/
/// Windows: C:\Users\Username\AppData\Local\walnut\noir-libs\Cache
pub fn get_cache_dir() -> Option<PathBuf> {
    ProjectDirs::from("com", "walnut", "noir-libs")
        .map(|proj_dirs| proj_dirs.cache_dir().to_path_buf())
}

fn ensure_dir(path: &Path) -> std::io::Result<()> {
    if path.exists() {
        if path.is_file() {
            return Err(io::Error::new(io::ErrorKind::AlreadyExists, "A file with the same name as the directory exists"));
        }
    } else {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

pub fn prepare_cache_dir() -> PathBuf {
    let cache_dir = get_cache_dir().expect("Could not determine cache directory");
    ensure_dir(&cache_dir).expect("Failed to setup cache directory");
    cache_dir
}

pub fn extract_package(path_with_version: &PathBuf, path_without_version : &PathBuf, version: &str) -> io::Result<PathBuf> {
    println!("WITH {:?} WITHOUT {:?} VER {:?}", path_with_version, path_without_version, version);

    let extract_dir = path_without_version.join(version);
    // Ensure the extract_dir is created
    // ensure_dir(&extract_dir)?;

    // Open the tar.gz file
    let tar_gz = File::open(path_with_version)?;
    let gz = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(gz);

    // Unpack the archive into the extract_dir
    archive.unpack(&extract_dir)?;

    Ok(extract_dir)
}