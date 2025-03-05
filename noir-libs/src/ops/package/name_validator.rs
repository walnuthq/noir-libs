use anyhow::bail;
use crate::config::MANIFEST_FILE_NAME;

// Name validation is done on backend side
// Here we perform very basic validation
pub fn validate_name_is_not_empty(name: &str) -> anyhow::Result<()> {
    // Must not be empty
    if name.trim().is_empty() {
        bail! {"package name {} in {} name is invalid. It cannot be empty.", &name, MANIFEST_FILE_NAME };
    }

    Ok(())
}