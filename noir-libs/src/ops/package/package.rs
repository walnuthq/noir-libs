use crate::config::MANIFEST_FILE_NAME;
use crate::filesystem::{copy_all, new_dir_replace_if_exists};
use crate::manifest::{read_manifest, Manifest, PackageType};
use crate::tar::create_tar_gz;
use anyhow::{bail, Result};
use indoc::formatdoc;
use std::path::PathBuf;
use crate::ops::package::name_validator::validate_name_is_not_empty;

pub struct PackagedTarball {
    pub tarball_path: String,
    pub name: String,
    pub version: String,
}

pub fn package(manifest_folder: &PathBuf, dst_folder: &PathBuf) -> Result<PackagedTarball> {
    let manifest: Manifest = read_manifest(&manifest_folder)?;
    verify_package_type_is_lib(&manifest)?;
    let version = verify_and_get_version(&manifest)?;
    let package_name = verify_and_get_package_name(&manifest)?;

    let temp_folder_name = format!("{}_{}", &package_name, &version);
    let temp_folder_path = &dst_folder.join(&temp_folder_name);

    let data_temp_folder_path = &dst_folder.join(&temp_folder_name).join(&temp_folder_name);
    new_dir_replace_if_exists(&temp_folder_path)?;

    copy_all(
        &manifest_folder,
        &data_temp_folder_path,
        &["target", ".cargo", ".vscode", &temp_folder_name],
        &[".env"],
    )?;

    let tarball_path = &temp_folder_path.join(format!("{}_{}.tar.gz", &package_name, &version));
    create_tar_gz(&data_temp_folder_path, &tarball_path)?;

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

fn verify_and_get_version(manifest: &Manifest) -> Result<String> {
    match &manifest.package.version {
        Some(version) => {
            match semver::Version::parse(version) {
                Ok(_) => Ok(version.to_string()),
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
            validate_name_is_not_empty(name)?;
            Ok(name)
        }
        None => {
            bail!(formatdoc! {
                "package name in {} file is not set. Please provide valid package name. Example:

                 [package]
                 name = \"my_example_package\"", &MANIFEST_FILE_NAME }
            );
        }
    }
}



#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use crate::ops::package::package::package;
    use crate::tar::extract_tar_gz;

    const TEST_MANIFEST_FOLDER_PATH: &str = "tests/test_files/test_noir_package";
    const PACKAGING_OUTPUT_FOLDER_PATH: &str = "tests/generated";

    // Used to remove generated artifacts after tests are done
    struct TestCleanup {
        path: PathBuf,
    }

    impl Drop for TestCleanup {
        fn drop(&mut self) {
            if self.path.exists() {
                fs::remove_dir_all(&self.path).expect("Failed to remove test directory");
                println!("Cleanup: Removed test directory {:?}", self.path);
            }
        }
    }

    #[test]
    fn test_package_command() {
        //given
        let current_dir = std::env::current_dir().unwrap();
        let manifest_folder = current_dir.join(TEST_MANIFEST_FOLDER_PATH);
        let packaging_dst_folder = current_dir.join(PACKAGING_OUTPUT_FOLDER_PATH);
        let _cleanup = TestCleanup { path: packaging_dst_folder.clone() };

        // when
        let packaged_tarball = package(&manifest_folder, &packaging_dst_folder).unwrap();

        // then
        assert_eq!(packaged_tarball.name, "my_cool_package");
        assert_eq!(packaged_tarball.version, "0.1.1");
        let expected_tarball_path = &packaging_dst_folder.join("my_cool_package_0.1.1/my_cool_package_0.1.1.tar.gz");
        assert_eq!(packaged_tarball.tarball_path, expected_tarball_path.to_str().unwrap());


        // when
        let extract_tarball_folder = packaging_dst_folder.join("extracted");
        extract_tar_gz(&expected_tarball_path.as_path(), extract_tarball_folder.as_path()).unwrap();

        // then
        let expected_file_path = &extract_tarball_folder.join("file_to_be_packaged.txt");
        let expected_file_path2 = &extract_tarball_folder.join("Nargo.toml");
        let expected_file_path3 = &extract_tarball_folder.join(".gitignore");
        let expected_file_path4 = &extract_tarball_folder.join("src/lib.rs");
        assert!(expected_file_path.exists());
        assert!(expected_file_path2.exists());
        assert!(expected_file_path3.exists());
        assert!(expected_file_path4.exists());

        // assert in my_cool_package_0.1.1 are only 4 files
        let extracted_tarball_files = fs::read_dir(extract_tarball_folder.as_path()).unwrap();
        let extracted_tarball_files_count = extracted_tarball_files.count();
        assert_eq!(4, extracted_tarball_files_count);

        // assert in src folder is only 1 file
        let extract_tarball_package_src_folder = &extract_tarball_folder.join("src");
        let extracted_tarball_package_src_files = fs::read_dir(extract_tarball_package_src_folder.as_path()).unwrap();
        let extracted_tarball_package_src_files_count = extracted_tarball_package_src_files.count();
        assert_eq!(1, extracted_tarball_package_src_files_count);
    }
}