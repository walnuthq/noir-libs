use anyhow::bail;
use indoc::formatdoc;
use crate::api::api::yank_package_api;
use crate::config::{API_KEY_ENV_VAR_NAME, REGISTRY_HOME_URL};

pub fn yank(package_name: &str, version: &str) -> anyhow::Result<()> {
    let api_key = match std::env::var(&API_KEY_ENV_VAR_NAME) {
        Ok(api_key) => api_key,
        Err(_e) => {
            bail!(formatdoc! {
                "Cannot yank a package. API KEY env variable not found.

                 Please generate an API KEY with \"yank\" scope  key at {}.

                 If you already have an API KEY, please export it before running yank command:
                 export {}=<your api key>
                 ", format!("{}/dashboard", &REGISTRY_HOME_URL), &API_KEY_ENV_VAR_NAME }
            );
        }
    };
    yank_package_api(package_name, version, api_key)
}