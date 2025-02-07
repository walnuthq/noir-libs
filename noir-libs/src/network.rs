use reqwest::blocking::{get};
use serde_json::Value;
use std::fs::File;
use std::io::{copy, Read};
use std::path::Path;
use anyhow::{bail, Result};
use crate::config::REGISTRY_URL;
use crate::ops::package::PackagedTarball;

/// Downloads a package from a remote URL and saves it to the specified output path.
///
/// # Arguments
///
/// * `output_path` - The path where the downloaded package will be saved.
/// * `url` - The url to download from
///
/// # Errors
///
/// This function will panic if the request fails, if the file cannot be created,
/// or if writing to the file fails.
pub fn download_remote(output_path: &Path, url: &str) -> Result<(), String> {
    let mut response = get(url).map_err(|e| e.to_string())?;
    //println!("RESPONSE {:?}", response);

    // Check if the response status is successful (200 OK)
    if !response.status().is_success() {
        return Err(format!(
            "Failed to download file: Received status code {}",
            response.status()
        ));
    }

    let mut dest = File::create(output_path).map_err(|e| e.to_string())?;
    copy(&mut response, &mut dest).map_err(|e| e.to_string())?;

    Ok(())
}

/// Retrieves the latest version of a package from the specified URL.
///
/// # Arguments
///
/// * `url` - A string containing the URL to fetch the latest version from.
///
/// # Returns
///
/// This function returns a string representing the latest version of the package.
///
/// # Errors
///
/// This function will panic if the request fails, if the response cannot be read,
/// or if the JSON cannot be parsed correctly.
pub fn get_latest_version(url: String) -> Result<String, String> {
    println!("Downloading latest package from url {}", url);

    let response = get(&url).map_err(|e| e.to_string())?;

    if response.status().is_success() {
        let aaa = &response.text().map_err(|e| e.to_string())?;
        let json: Value = serde_json::from_str(aaa).map_err(|e| e.to_string())?;

        Ok(json["latest_version"]
            .as_str()
            .ok_or("Version field not found or is not a string")?
            .to_string())
    } else {
        let error_message = response.text().map_err(|e| e.to_string())?;
        let json_error: Value = serde_json::from_str(&error_message).unwrap_or_default();
        let message = json_error
            .get("message")
            .and_then(Value::as_str)
            .unwrap_or("Unknown error");

        Err(message.to_string())
    }
}

/// Retrieves the latest version of a package from the specified URL.
///
/// # Arguments
///
/// * `url` - A string containing the URL to fetch the latest version from.
///
/// # Returns
///
/// This function returns a string representing the latest version of the package.
///
/// # Errors
///
/// This function will panic if the request fails, if the response cannot be read,
/// or if the JSON cannot be parsed correctly.
pub fn publish_package(packaged_tarball: &PackagedTarball) -> Result<()> {
    let package_path = Path::new(&packaged_tarball.tarball_path);
    let name = &packaged_tarball.name.as_str();
    let version = &packaged_tarball.version.as_str();

    // Check if the packed file exists
    if !package_path.exists() {
        bail!(format!("Packed file does not exist: {}", &package_path.to_path_buf().display()));
    }

    // Open the file synchronously
    let mut file = std::fs::File::open(&package_path)?;

    let mut buffer = Vec::new();
    // Read the file into the buffer
    file.read_to_end(&mut buffer)?;

    let length = buffer.len();

    let file_part = reqwest::blocking::multipart::Part::bytes(buffer)
        .file_name(format!("{}_{}", &name, &version))
        .mime_str("application/gzip")?;

    let form = reqwest::blocking::multipart::Form::new().part("file", file_part);
    println!("Buffer length: {}", length);

    // Send the request synchronously
    let client = reqwest::blocking::Client::new();
    match client
        .post(format!("{}/packages/{}/{}/upload", REGISTRY_URL, &name, &version))
        .multipart(form)
        .send() {
        Ok(response) => {
            // Optionally, check the response here
            if response.status().is_success() {
                println!("Successfully uploaded package: {}", &name);
            } else {
                println!("Failed to upload package: {}. Status: {}", &name, response.status())
            }
        }
        Err(err) => {
            println!("Failed to upload package: {}. Error: {}", &name, err);
        }
    }
    Ok(())

}


#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_download_remote() {
        let mut server = mockito::Server::new();

        let url = server.url();

        // Create a mock
        let mock = server
            .mock("GET", "/hello")
            .with_status(201)
            .with_body("world")
            .create();

        let temp_dir = tempdir().expect("Failed to create temp dir");
        let output_path = temp_dir.path().join("test_file");
        let url_str = format!("{}/hello", url);
        download_remote(&output_path, &url_str).unwrap();

        assert!(output_path.is_file());

        mock.assert();
    }

    #[test]
    fn test_get_latest_version() {
        let mut server = mockito::Server::new();

        let url = server.url();

        // Create a mock
        let mock = server
            .mock("GET", "/latest")
            .with_status(200)
            .with_body(r#"{"latest_version": "1.2.3"}"#)
            .create();

        let url = format!("{}/latest", url);
        let version = get_latest_version(url).unwrap();

        assert_eq!(version, "1.2.3");
        mock.assert();
    }

    #[test]
    fn test_get_latest_version_not_found() {
        let mut server = mockito::Server::new();

        let url = server.url();

        // Create a mock
        let mock = server.mock("GET", "/latest").with_status(404).create();

        let url = format!("{}/latest", url);

        let result = std::panic::catch_unwind(|| {
            get_latest_version(url).unwrap();
        });

        assert!(result.is_err());
        mock.assert();
    }
}
