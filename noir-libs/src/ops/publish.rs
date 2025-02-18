use crate::api::api::publish_package_api;
use crate::config::PACKAGING_OUTPUT_FOLDER_PATH;
use crate::ops::package::package::package;
use anyhow::Result;

pub fn publish() -> Result<String> {
    let current_dir = std::env::current_dir()?;
    let output_folder_path = &current_dir.join(PACKAGING_OUTPUT_FOLDER_PATH);

    let tarball_path = package(&current_dir, &output_folder_path)?;
    let result_message = publish_package_api(&tarball_path)?;
    Ok(result_message)
}