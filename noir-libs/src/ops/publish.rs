use anyhow::Result;
use crate::api::api::publish_package_api;
use crate::ops::package::package;

pub fn publish() -> Result<(String)> {
    let tarball_path = package()?;
    let result_message = publish_package_api(&tarball_path)?;
    Ok((result_message))
}