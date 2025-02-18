use anyhow::bail;
use crate::config::MANIFEST_FILE_NAME;

const RESERVED_NAMES: &[&str] = &[
    "nul", "con", "prn", "aux", "clock$", "com1", "com2", "com3", "com4",
    "com5", "com6", "com7", "com8", "com9", "lpt1", "lpt2", "lpt3", "lpt4",
    "lpt5", "lpt6", "lpt7", "lpt8", "lpt9"
];

// Name validation is inspired by crates.io specification https://doc.rust-lang.org/cargo/reference/manifest.html#the-name-field
// Rules:
// - maximum of 64 characters of length and not empty
// - only alphanumeric characters are allowed, except for special characters: "-" and "_"
// - cannot start with special character
// - can contain only single special characters between alphanumeric characters (e.g. no "--" or "__")
// - do not use special Windows names such as “nul”
pub fn validate_name(name: &str) -> anyhow::Result<()> {
    // Must not be empty
    if name.is_empty() {
        bail! {"package name {} in {} name is invalid. It cannot be empty.", &name, MANIFEST_FILE_NAME };
    }

    // Must be at most 64 characters
    if name.len() > 64 {
        bail! {"package name {} in {} name is invalid. It must be maximum of 64 characters long.", &name, MANIFEST_FILE_NAME };
    }

    // First character must be alphanumeric
    let mut chars = name.chars();
    if let Some(first_char) = chars.next() {
        if !first_char.is_ascii_alphanumeric() {
            bail! {"package name {} in {} name is invalid. First character must be alphanumeric.", &name, MANIFEST_FILE_NAME };
        }
    }

    // Last character must be alphanumeric
    if let Some(last_char) = name.chars().last() {
        if !last_char.is_ascii_alphanumeric() {
            bail! {"package name {} in {} name is invalid. Last character must be alphanumeric.", &name, MANIFEST_FILE_NAME };
        }
    }

    // Must contain only alphanumeric characters, `_`, or `-`
    for c in name.chars() {
        if !c.is_ascii_alphanumeric() && c != '_' && c != '-' {
            bail! {"package name {} in {} name is invalid. Can contain only alphanumeric characters, '-' or '_'.", &name, MANIFEST_FILE_NAME };
        }
    }

    // Can contain only single special characters between alphanumeric characters
    let special_characters_correct = only_contains_single_special_characters(&name);
    if !special_characters_correct {
        bail! {"package name {} in {} name is invalid. Can contain only alphanumeric characters, '-' or '_'.", &name, MANIFEST_FILE_NAME };
    }

    // Must not be a reserved name (case insensitive)
    let lowercase_name = name.to_ascii_lowercase();
    if RESERVED_NAMES.contains(&lowercase_name.as_str()) {
        bail! {"package name {} in {} name is invalid. It is reserved value and cannot be used as name.", &name, MANIFEST_FILE_NAME };
    }

    Ok(())
}

fn only_contains_single_special_characters(name: &str) -> bool {
    let mut chars = name.chars().peekable();
    let mut prev_was_separator = false;
    let mut is_correct = true;
    while let Some(c) = chars.next() {
        if c == '-' || c == '_' {
            if prev_was_separator {
                is_correct = false;
                break;
            }
            prev_was_separator = true;
        } else if c.is_ascii_alphanumeric() {
            prev_was_separator = false;
        } else {
            is_correct = false;
            break;
        }
    }
    is_correct
}
#[cfg(test)]
mod tests {
    use crate::ops::package::name_validator::validate_name;

    #[test]
    fn test_validate_name() -> anyhow::Result<()> {
        let long_name = "a".repeat(64);

        let valid_names = [
            "serde",
            "serde_json",
            "my-crate",
            "some_package_2",
            "hello_world",
            "valid123",
            "valid-name-123",
            long_name.as_str(),
        ];

        for name in valid_names.iter() {
            assert!(
                validate_name(name).is_ok(),
                "Expected '{}' to be valid, but it was rejected",
                name
            );
        }

        let invalid_names = [
            "nul", "con", "prn", "lpt1", // Windows reserved names
            "-serde", "_serde",          // Cannot start with `-` or `_`
            "serde-", "serde_",          // Cannot end with `-` or `_`
            "serde--json", "serde__json", // Double `--` or `__` not allowed
            "my--cool-package", "my_cool__package", // Double `--` or `__` not allowed
            "some@crate", "crate!", "crate.lib",      // Invalid characters
            "this_is_a_really_long_package_name_that_exceeds_sixty_four_characters", // Too long
        ];

        for name in invalid_names.iter() {
            assert!(
                validate_name(name).is_err(),
                "Expected '{}' to be invalid, but it was accepted",
                name
            );
        }

        Ok(())
    }
}