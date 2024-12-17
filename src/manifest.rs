use std::{
    env,
    path::{Path, PathBuf},
};

use toml_edit::DocumentMut;

use crate::MANIFEST_FILE_NAME;

pub fn write_package_dep(project_dir: PathBuf, package_name: &str, path: &str) -> PathBuf {
    println!("Adding value {}", path);

    // let pwd = env::current_dir().expect("Unable to find current folder");
    let manifest = try_find_manifest(&project_dir).unwrap();
    let content = std::fs::read_to_string(manifest.clone()).expect("Cannot read file");
    let mut doc = content.parse::<DocumentMut>().expect("Invalid TOML");
    //doc["dependencies"][package_name] = value(path);

    let mut table = toml_edit::InlineTable::default();
    table.get_or_insert("path", path);
    // Assign to the dependencies table
    doc["dependencies"][package_name] =
        toml_edit::Item::Value(toml_edit::Value::InlineTable(table));

    std::fs::write(manifest.clone(), doc.to_string()).expect("Cannot write file");

    manifest
}

pub fn remove_package(package_name: &str) {
    println!("Removing package {}", package_name);
    let pwd = env::current_dir().expect("Unable to find current folder");
    let manifest = try_find_manifest(&pwd).unwrap();

    // Read the file content
    let content = std::fs::read_to_string(&manifest).expect("Cannot read file");

    // Parse the content as TOML
    let mut doc = content.parse::<DocumentMut>().expect("Invalid TOML");

    // Ensure that the "dependencies" table exists and is mutable
    if let Some(dependencies) = doc["dependencies"].as_table_mut() {
        // Remove the specified package from the dependencies
        dependencies.remove(package_name);
    }

    // Write the modified content back to the file
    std::fs::write(manifest, doc.to_string()).expect("Cannot write file");
}

pub fn get_dependencies(manifest: PathBuf) -> Vec<(String, String)> {
    // Read the file content
    let content = std::fs::read_to_string(&manifest).expect("Cannot read file");

    // Parse the content as TOML
    let doc = content.parse::<DocumentMut>().expect("Invalid TOML");

    // Initialize a Vec to store dependencies and versions
    let mut dependencies = Vec::new();

    // Ensure that the "dependencies" table exists
    if let Some(deps) = doc.get("dependencies").and_then(|d| d.as_table()) {
        for (key, value) in deps.iter() {
            //println!("Found {} ja {}", key, value);
            if let Some(path) = value.get("path").and_then(|p| p.as_str()) {
                //  println!("On path {}", path);
                if let Some(version) = extract_version_from_path(path) {
                    //    println!("On version {}", version);
                    dependencies.push((key.to_string(), version));
                }
            }
        }
    }

    dependencies
}

fn extract_version_from_path(path: &str) -> Option<String> {
    let filename = path.split('/').last()?;
    Some(filename.to_string())
}

pub fn try_find_manifest(start_dir: &Path) -> Option<PathBuf> {
    let mut root = Some(start_dir);
    while let Some(path) = root {
        let manifest = path.join(MANIFEST_FILE_NAME);
        if manifest.is_file() {
            return Some(manifest);
        }
        root = path.parent();
    }
    None
}
