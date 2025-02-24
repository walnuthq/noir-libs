use crate::api::api::publish_package_api;
use crate::config::{API_KEY_ENV_VAR_NAME, PACKAGING_OUTPUT_FOLDER_PATH, REGISTRY_HOME_URL};
use crate::ops::package::package::package;
use anyhow::{bail, Result};
use indoc::formatdoc;

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
    let current_dir = std::env::current_dir()?;
    let output_folder_path = &current_dir.join(PACKAGING_OUTPUT_FOLDER_PATH);

    let tarball_path = package(&current_dir, &output_folder_path)?;
    let result_message = publish_package_api(&tarball_path, api_key)?;
    Ok(result_message)
}