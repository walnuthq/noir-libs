use std::collections::HashMap;
use anyhow::{Context, Result};
use serde::Deserialize;
use std::fmt;
use std::path::{Path, PathBuf};
use toml_edit::DocumentMut;

use crate::config::MANIFEST_FILE_NAME;

#[derive(Debug, Deserialize)]
pub struct Manifest {
    pub package: Package,
    pub dependencies: HashMap<String, Dependency>,
}

#[derive(Debug, Deserialize)]
pub struct Package {
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub package_type: Option<PackageType>,
    pub compiler_version: Option<String>,
    pub authors: Option<Vec<String>>,
    pub version: Option<String>,
    pub description: Option<String>,
    pub license: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub documentation: Option<String>,
    pub repository: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Dependency {
    Git {
        git: String,
        tag: Option<String>,
        directory: Option<String>,
    },
    Path {
        path: String,
    },
}


#[derive(Debug, Deserialize, PartialEq)]
pub enum PackageType {
    #[serde(rename = "lib")]
    Library,
    #[serde(rename = "contract")]
    Contract,
    #[serde(rename = "bin")]
    Binary,
}

impl fmt::Display for PackageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PackageType::Library => write!(f, "lib"),
            PackageType::Contract => write!(f, "contract"),
            PackageType::Binary => write!(f, "bin"),
        }
    }
}

pub fn read_manifest(project_dir: &PathBuf) -> Result<Manifest> {
    let manifest = get_manifest(&project_dir).with_context(|| format!("Unable to find {} manifest file. Please verify you are in the correct directory.", &MANIFEST_FILE_NAME))?;
    let content = std::fs::read_to_string(manifest.clone()).with_context(|| format!("Cannot read {} manifest file. File {} was found but cannot be read.", &MANIFEST_FILE_NAME, manifest.to_str().unwrap()))?;
    let doc: toml::Value  = toml::from_str(&content).with_context(|| format!("{} manifest file is invalid TOML.", manifest.to_str().unwrap()))?;
    let manifest: Manifest = doc.try_into().with_context(|| format!("Failed to parse {} manifest file. Assure file has all required properties.", &MANIFEST_FILE_NAME))?;
    Ok(manifest)
}

/// Writes a package dependency to the specified TOML manifest file.
///
/// # Arguments
///
/// * `project_dir` - The directory containing the manifest file.
/// * `package_name` - The name of the package to be added as a dependency.
/// * `path` - The path to the package.
///
/// # Returns
///
/// A `PathBuf` representing the path to the updated manifest file.
///
/// # Errors
///
/// This function will panic if the manifest file cannot be found, if the file cannot be read,
/// or if the content is not valid TOML.
pub fn write_package_dep(manifest_path: &PathBuf, package_name: &str, path: &str) -> PathBuf {
    let content = std::fs::read_to_string(manifest_path.clone()).expect(format!("Cannot read {} manifest file", &MANIFEST_FILE_NAME).as_str());
    let mut doc = content.parse::<DocumentMut>().expect("Invalid TOML");

    // Ensure the "dependencies" table exists
    let dependencies = doc
        .entry("dependencies")
        .or_insert_with(|| toml_edit::Item::Table(toml_edit::Table::new()));

    let mut table = toml_edit::InlineTable::default();
    table.get_or_insert("path", path);
    // Assign to the dependencies table
    dependencies[package_name] = toml_edit::Item::Value(toml_edit::Value::InlineTable(table));

    std::fs::write(manifest_path.clone(), doc.to_string()).expect("Cannot write file");

    manifest_path.clone()
}

/// Retrieves the dependencies and their versions from the specified TOML manifest file.
///
/// # Arguments
///
/// * `manifest` - Path to the manifest file.
///
/// # Returns
///
/// A vector of tuples, where each tuple contains the name of the dependency as a `String`
/// and its corresponding version as a `String`.
///
/// # Errors
///
/// This function will panic if the file cannot be read or if the content is not valid TOML.
pub fn get_dependencies(manifest: PathBuf) -> Vec<(String, String)> {
    // Read the file content
    let content = std::fs::read_to_string(&manifest).expect("Cannot read file");

    // Parse the content as TOML
    let doc = content.parse::<DocumentMut>().expect("Invalid TOML");

    // Ensure that the "dependencies" table exists and map dependencies to a vector
    if let Some(deps) = doc.get("dependencies").and_then(|d| d.as_table()) {
        return deps
            .iter()
            .filter_map(|(key, value)| {
                value.get("path").and_then(|p| p.as_str()).and_then(|path| {
                    extract_version_from_path(path).map(|version| (key.to_string(), version))
                })
            })
            .collect();
    }

    Vec::new()
}

/// Tries to find the TOML manifest file starting from the given directory.
///
/// # Arguments
///
/// * `start_dir` - The dir from which to start the search.
///
/// # Returns
///
/// An `Option<PathBuf>` that contains the path to the manifest file if found, or `None` if not found.
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

/// Reads TOML manifest file in the given directory.
///
/// # Arguments
///
/// * `dir` - The dir where to read.
///
/// # Returns
///
/// An `Option<PathBuf>` that contains the path to the manifest file if found, or `None` if not found.
fn get_manifest(dir: &Path) -> Option<PathBuf> {
    let manifest = dir.join(MANIFEST_FILE_NAME);
    if manifest.is_file() {
        Some(manifest)
    } else {
        None
    }
}

fn extract_version_from_path(path: &str) -> Option<String> {
    if path.is_empty() {
        return None; // Return None if the path is empty
    }

    let filename = path.split('/').last()?;
    Some(filename.to_string())
}

/// Removes a specified package from the dependencies in the manifest file.
///
/// # Arguments
///
/// * `dir` - Directory containing the TOML manifest file.
/// * `package_name` - Name of the package to be removed.
///
/// # Errors
///
/// This function will panic if the manifest file cannot be found or if the file cannot be read or written.
pub fn remove_package(dir: PathBuf, package_name: &str) {
    let manifest = try_find_manifest(&dir).unwrap();

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use crate::config::DEPENDENCIES_FOLDER_NAME;

    #[test]
    fn test_write_package_dep() {
        let temp_dir = tempfile::tempdir().unwrap();
        let project_dir = temp_dir.path().to_path_buf();
        let manifest_path = project_dir.join(MANIFEST_FILE_NAME);

        // Create an initial empty TOML file with only dependencies section
        fs::write(&manifest_path, "[dependencies]\n").unwrap();

        // Call the function to test
        let package_name = "my_package";
        let path = format!("{}/my_package/0.1.0", DEPENDENCIES_FOLDER_NAME);
        let result = write_package_dep(&manifest_path, package_name, path.as_str());

        // Verify that the manifest file was updated correctly
        let content = fs::read_to_string(result).unwrap();
        assert!(content.contains("my_package"));
        assert!(content.contains(format!("path = \"{}\"", path).as_str()));

        // Cleanup
        temp_dir.close().unwrap();
    }

    #[test]
    fn test_write_package_dep_missing_deps_section() {
        let temp_dir = tempfile::tempdir().unwrap();
        let project_dir = temp_dir.path().to_path_buf();
        let manifest_path = project_dir.join(MANIFEST_FILE_NAME);

        // Create an initial empty TOML file
        fs::write(&manifest_path, "\n").unwrap();

        // Call the function to test
        let package_name = "my_package";
        let path = "../../my_package/0.1.0";
        let result = write_package_dep(&manifest_path, package_name, path);

        // Verify that the manifest file was updated correctly
        let content = fs::read_to_string(result).unwrap();
        assert!(content.contains("[dependencies]"));
        assert!(content.contains("my_package"));
        assert!(content.contains("path = \"../../my_package/0.1.0\""));

        // Cleanup
        temp_dir.close().unwrap();
    }

    #[test]
    fn test_get_dependencies() {
        let temp_dir = tempfile::tempdir().unwrap();
        let project_dir = temp_dir.path().to_path_buf();
        let manifest_path = project_dir.join(MANIFEST_FILE_NAME);

        // Create a TOML file with dependencies
        fs::write(
            &manifest_path,
            "[dependencies]\nmy_package = { path = \"../../my_package/0.1.0\" }\n
            my_package2 = { path = \"../../my_package2/0.1.2\" }\n",
        )
        .unwrap();

        // Call the function to test
        let dependencies = get_dependencies(manifest_path.clone());

        // Verify that the dependencies were read correctly
        assert_eq!(dependencies.len(), 2);
        assert_eq!(dependencies[0].0, "my_package");
        assert_eq!(dependencies[0].1, "0.1.0");
        assert_eq!(dependencies[1].0, "my_package2");
        assert_eq!(dependencies[1].1, "0.1.2");

        // Cleanup
        temp_dir.close().unwrap();
    }

    #[test]
    fn test_try_find_manifest() {
        let temp_dir = tempfile::tempdir().unwrap();
        let project_dir = temp_dir.path().to_path_buf();
        let manifest_path = project_dir.join(MANIFEST_FILE_NAME);

        // Create an empty TOML file
        fs::write(&manifest_path, "").unwrap();

        // Call the function to test
        let found_manifest = try_find_manifest(project_dir.as_path());
        assert!(found_manifest.is_some());
        assert_eq!(found_manifest.unwrap(), manifest_path);

        let found_manifest = try_find_manifest(project_dir.join("newdir").as_path());
        assert!(found_manifest.is_some());
        assert_eq!(found_manifest.unwrap(), manifest_path);

        // Cleanup
        temp_dir.close().unwrap();
    }

    #[test]
    fn test_remove_package() {
        let temp_dir = tempfile::tempdir().unwrap();
        let project_dir = temp_dir.path().to_path_buf();
        let manifest_path = project_dir.join(MANIFEST_FILE_NAME);

        // Create a TOML file with a dependency
        fs::write(
            &manifest_path,
            "[dependencies]\nmy_package = { path = \"../../my_package/0.1.0\" }\n",
        )
        .unwrap();

        // Call the function to test
        remove_package(project_dir, "my_package");

        let content = fs::read_to_string(manifest_path).unwrap();
        assert!(!content.contains("my_package"));

        // Cleanup
        temp_dir.close().unwrap();
    }
}
