use crate::config::REGISTRY_HOME_URL;
use crate::ops::package::package::PackagedTarball;
use crate::path::get_full_package_name;
use anyhow::bail;
use indoc::formatdoc;
use reqwest::blocking::get;
use serde::Deserialize;
use serde_json::Value;
use std::fs::File;
use std::io::{copy, Read};
use std::path::Path;

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
pub fn download_package(output_path: &Path, url: &str) -> Result<(), String> {
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

#[derive(Debug, Deserialize)]
struct Version {
    version: String,
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
pub fn get_latest_package_version(url: String) -> Result<String, String> {
    println!("Downloading latest package from url {}", url);

    let response = get(&url).map_err(|e| e.to_string())?;

    if response.status().is_success() {
        let resp_text = &response.text().map_err(|e| e.to_string())?;
        let json: Version = serde_json::from_str(resp_text).map_err(|e| e.to_string())?;
        Ok(json.version)
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

/// Publishes a package to the remote registry.
///
/// # Arguments
///
/// * `packaged_tarball` - a GZIP compressed tarball of the package to be published.
/// * `api_key` - the API key to authenticate the user generated in the remote registry.
/// * `url` - A string containing the URL to publish a package.
///
/// # Returns
///
/// This function returns a string with success message to display to the user.
///
/// # Errors
///
/// This function return an error when package tarball not exists or when the request fails.
pub fn publish_package(packaged_tarball: &PackagedTarball, api_key: String, url: String) -> anyhow::Result<String> {
    let package_path = Path::new(&packaged_tarball.tarball_path);
    // Check if the packed file exists
    if !package_path.exists() {
        bail!(format!("Packed file does not exist: {}", &package_path.to_path_buf().display()));
    }

    let name = packaged_tarball.name.as_str();
    let version = packaged_tarball.version.as_str();

    let mut file = File::open(&package_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let file_part = reqwest::blocking::multipart::Part::bytes(buffer)
        .file_name(get_full_package_name(name, version))
        .mime_str("application/gzip")?;

    let form = reqwest::blocking::multipart::Form::new().part("file", file_part);

    let client = reqwest::blocking::Client::new();
    match client
        .post(url)
        .multipart(form)
        .header("Authorization", get_auth_header(&api_key))
        .send() {
            Ok(response) => {
                if response.status().is_success() {
                    Ok(formatdoc! { "Successfully published package: {} {} to noir-libs registry.
                    Explore your package at: {}/packages/{}/{}", &name, &version, &REGISTRY_HOME_URL, &name, &version})
                } else if response.status().is_client_error() {
                    let err_message = get_error_message_from_response(response);
                    bail!("Failed to upload package: {} {}. Error message: {}", &name, &version, &err_message)
                } else {
                    bail!("Failed to upload package: {} {}. Server status: {}", &name, &version, response.status())
                }
            }
            Err(err) => {
                bail!("Failed to upload package: {} {}. Error: {}", &name, &version, err);
            }
    }
}

fn get_auth_header(api_key_string: &str) -> String {
    format!("Bearer {}", api_key_string)
}

fn get_error_message_from_response(response: reqwest::blocking::Response) -> String {
    match response.json::<Value>() {
        Ok(json) => json.get("message")
            .and_then(|m| m.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "Unknown error".to_string()),
        Err(_) => "Unknown error".to_string(),
    }
}

pub fn yank_package(name: &str, version: &str, api_key: String, url: String) -> anyhow::Result<()> {
    let client = reqwest::blocking::Client::new();
    match client
        .put(&url)
        .header("Authorization", get_auth_header(&api_key))
        .send() {
            Ok(response) => {
                if response.status().is_success() {
                    Ok(())
                } else if response.status().is_client_error() {
                    let err_message = get_error_message_from_response(response);
                    bail!("Failed to yank a package: {} {}. Error message: {}", &name, &version, &err_message)
                } else {
                    bail!("Failed to yank a package: {} {}. Server status: {}", &name, &version, response.status())
                }
            }
            Err(err) => {
                bail!("Failed to yank a package: {} {}. Error: {}", &name, &version,  err);
            }
        }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Error;
    use mockito::{Matcher, Mock, ServerGuard};
    use std::io::Write;
    use std::path::PathBuf;
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
        download_package(&output_path, &url_str).unwrap();

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
            .with_body(r#"{"version": "1.2.3"}"#)
            .create();

        let url = format!("{}/latest", url);
        let version = get_latest_package_version(url).unwrap();

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
            get_latest_package_version(url).unwrap();
        });

        assert!(result.is_err());
        mock.assert();
    }

    #[test]
    fn test_publish_package_success() -> anyhow::Result<()> {
        let (tarball_path, _server, mock, result) = publish_package_to_mock(200)?;

        assert!(result.is_ok());
        assert!(result.unwrap().contains("Successfully published package"));
        mock.assert();

        std::fs::remove_file(&tarball_path).ok(); // Ensure the file is deleted

        Ok(())
    }

    #[test]
    fn test_publish_package_bad_request_error() -> anyhow::Result<()> {
        test_publish_package_with_error(400)?
    }

    #[test]
    fn test_publish_package_server_error() -> anyhow::Result<()> {
        test_publish_package_with_error(500)?
    }

    fn test_publish_package_with_error(status_to_return: usize) -> Result<Result<(), Error>, Error> {
        let (tarball_path, _server, mock, result) = publish_package_to_mock(status_to_return)?;

        assert!(result.is_err());
        assert!(result.err().unwrap().to_string().contains("Failed to upload package"));
        mock.assert();

        std::fs::remove_file(&tarball_path).ok(); // Ensure the file is deleted

        Ok(Ok(()))
    }

    fn publish_package_to_mock(status_to_return: usize) -> anyhow::Result<(PathBuf, ServerGuard, Mock, anyhow::Result<String>)> {
        let temp_dir = tempdir()?;
        let tarball_path = temp_dir.path().join("test_package.tar.gz");

        {
            let mut file = File::create(&tarball_path)?;
            file.write_all(b"dummy content")?;
        }

        let package = PackagedTarball {
            tarball_path: tarball_path.to_str().unwrap().to_string(),
            name: "test_package".to_string(),
            version: "1.0.0".to_string(),
        };

        let api_key = "test_api_key".to_string();

        let mut server = mockito::Server::new();
        let url = server.url();

        let publish_url = format!("{}/publish", url);

        let mock = server.mock("POST", "/publish")
            .match_header("Authorization", Matcher::Any)
            .match_body(Matcher::Any)
            .with_status(status_to_return)
            .create();

        let result = publish_package(&package, api_key, publish_url);
        Ok((tarball_path, server, mock, result))
    }
}
