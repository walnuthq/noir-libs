use crate::config::MANIFEST_FILE_NAME;
use crate::filesystem::{copy_all, new_dir_replace_if_exists};
use crate::manifest::{read_manifest, Manifest, PackageType};
use crate::tar::{create_tar_gz, extract_tar_gz};
use anyhow::{bail, Result};
use indoc::formatdoc;
use regex::Regex;

pub struct PackagedTarball {
    pub tarball_path: String,
    pub name: String,
    pub version: String,
}

pub fn package() -> Result<(PackagedTarball)> {
    let current_dir = std::env::current_dir()?;
    let manifest: Manifest = read_manifest(&current_dir)?;
    verify_package_type_is_lib(&manifest)?;
    let version =verify_and_get_version(&manifest)?;
    let package_name = verify_and_get_package_name(&manifest)?;

    let temp_folder_name = format!("{}_temp", &package_name);
    let temp_folder_path = current_dir.join("target/package").join(&temp_folder_name);
    new_dir_replace_if_exists(&temp_folder_path)?;

    copy_all(
        &current_dir,
        &temp_folder_path,
        &["target", ".cargo", ".git", ".vscode", ".gitignore", ".git", &temp_folder_name],
        &[".env"],
    )?;

    let tarball_path = &temp_folder_path.join(format!("{}_{}.tar.gz", &package_name, &version));
    create_tar_gz(&temp_folder_path, &tarball_path)?;
    extract_tar_gz(&tarball_path, &current_dir.join("target/package/extracted_bro"))?;

    Ok(PackagedTarball {
        tarball_path: tarball_path.to_str().unwrap().to_string(),
        name: package_name.clone(),
        version: version.clone(),
    })
}

fn verify_package_type_is_lib(manifest: &Manifest) -> Result<()> {
    match &manifest.package.package_type {
        Some(package_type) => {
            if *package_type == PackageType::Library {
                Ok(())
            } else {
                bail!(formatdoc! {
                    "package type {} in {} file is incorrect. Assure package type is set to '{}'. Example:

                     [package]
                     type = \"{}\"", &package_type, &MANIFEST_FILE_NAME, PackageType::Library, PackageType::Library }
                );
            }
        }
        None => {
            bail!(formatdoc! {
                "package type in {} file is not set. Assure package type is set to '{}'. Example:

                 [package]
                 type = \"{}\"", &MANIFEST_FILE_NAME, PackageType::Library, PackageType::Library }
            );
        }
    }
}

fn verify_and_get_version(manifest: &Manifest) -> Result<(String)> {
    match &manifest.package.version {
        Some(version) => {
            match semver::Version::parse(version) {
                Ok(_) => Ok((version.to_string())),
                Err(_) => {
                    bail!(formatdoc! {
                        "package version {} in {} file is incorrect. Assure correct semantic versioning value. Example:

                         [package]
                         version = \"0.0.1\"", &version , &MANIFEST_FILE_NAME }
                    );
                }
            }
        }
        None => {
            bail!(formatdoc! {
                "package version in {} file is not set. Assure correct semantic versioning value. Example:

                 [package]
                 version = \"0.0.1\"", &MANIFEST_FILE_NAME }
            );
        }
    }
}

fn verify_and_get_package_name(manifest: &Manifest) -> Result<&String> {
    match &manifest.package.name {
        Some(name) => {
            validate_name(name)?;
            Ok(name)
        }
        None => {
            bail!(formatdoc! {
                "package name in {} file is not set. Assure correct semantic versioning value. Example:

                 [package]
                 name = \"my_example_package\"", &MANIFEST_FILE_NAME }
            );
        }
    }
}

fn validate_name(name: &str) -> Result<()> {
    let is_name_correct = match Regex::new(r"^(?:[a-z0-9]+(?:[-_][a-z0-9]+)*)(?:\.[a-z0-9]+(?:[-_][a-z0-9]+)*)*$")
        .map_err(|_| "Invalid regex pattern") {
        Ok(regex) => regex.is_match(name),
        Err(_) => false,
    };

    if !is_name_correct {
        return bail!(formatdoc! {"package name {} in {} name is invalid. Assure it follows the naming convention.",
            &name, &MANIFEST_FILE_NAME});
    }

    Ok(())
}
