use std::path::Path;
use colored::Colorize;
use crate::config::MANIFEST_FILE_NAME;
use crate::filesystem::prepare_cache_dir;
use crate::manifest::{read_manifest, Dependency, Manifest};

pub fn fetch() -> anyhow::Result<()> {
    let current_dir = std::env::current_dir()?;
    let manifest_path = crate::manifest::try_find_manifest(&current_dir).expect(format!("Unable to find {} manifest file", &MANIFEST_FILE_NAME).as_str());
    let manifest: Manifest = read_manifest(&current_dir)?;
    let manifest_dir = Path::new(&manifest_path)
        .parent()
        .expect("Failed to get manifest parent directory");
    let cache_root = prepare_cache_dir(&manifest_dir.to_path_buf());

    for dependency in manifest.dependencies.values() {
        match dependency {
            Dependency::Path { path } => {
                let path = Path::new(path.as_str());
                let version = path.file_name().expect("Path is incorrect")
                    .to_str().expect("Path file name is not valid UTF-8");
                let package_name = path.parent().expect("Path is incorrect").file_name().expect("Path is incorrect")
                    .to_str().expect("Path file name is not valid UTF-8");
                // Fetching packages should include downloading yanked packages for making current dependencies setup working
                if let Err(e) = crate::ops::add::store_package(cache_root.clone(), package_name, &version, true) {
                    println!("{}", format!("Fetching dependency {}@{} failed: {}", package_name, version, e).red().bold());
                }
            },
            Dependency::Git { .. } => {}
        }
    }

    Ok(())
}