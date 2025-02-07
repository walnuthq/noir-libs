use anyhow::Result;
use crate::network::publish_package;
use crate::ops::package::package;

pub fn publish() -> Result<()> {
    let tarball_path = package()?;
    publish_package(&tarball_path)?;
    Ok(())
}