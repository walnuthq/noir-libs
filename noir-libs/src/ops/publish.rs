use crate::api::api::publish_package_api;
use crate::config::{API_KEY_ENV_VAR_NAME, PACKAGING_OUTPUT_FOLDER_PATH, REGISTRY_HOME_URL};
use crate::manifest::{read_manifest, Manifest};
use crate::ops::package::package::{verify_and_get_package_name, verify_and_get_version, PackagedTarball};
use anyhow::{bail, Result};
use indoc::formatdoc;
use std::path::PathBuf;

pub fn publish() -> Result<String> {
    let api_key = match std::env::var(&API_KEY_ENV_VAR_NAME) {
        Ok(api_key) => api_key,
        Err(_e) => {
            bail!(formatdoc! {
                "Cannot publish a package. API KEY env variable not found.

                 Please generate an API KEY with \"publish\" scope  key at {}.

                 If you already have an API KEY, please export it before running publish command:
                 export API_KEY=<your api key>
                 ", format!("{}/dashboard", &REGISTRY_HOME_URL) }
            );
        }
    };

    let tarball_path = verify_tarball_existence()?;
    let result_message = publish_package_api(&tarball_path, api_key)?;
    Ok(result_message)
}

fn verify_tarball_existence() -> Result<PackagedTarball> {
    let current_dir = std::env::current_dir()?;
    let manifest: Manifest = read_manifest(&current_dir)?;
    let output_folder_path = &current_dir.join(PACKAGING_OUTPUT_FOLDER_PATH);
    let version = verify_and_get_version(&manifest)?;
    let package_name = verify_and_get_package_name(&manifest)?;
    let package = format!("{}_{}", &package_name, &version);
    let tarball_path = output_folder_path.join(&package).join(format!("{}.tar.gz", &package));
    println!("Tarball path: {}", tarball_path.display());
    if !tarball_path.exists() {
        bail!("Package tarball does not exists. Please run \"noir-libs package\" first.");
    }
    Ok(PackagedTarball {
        tarball_path: tarball_path.to_str().unwrap().to_string(),
        name: package_name.to_string(),
        version: version.to_string(),
    })
}
