use reqwest::blocking::get;
use std::fs::File;
use std::io::copy;
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
pub fn download_remote(output_path: &Path, url: &str) {
    println!("Downloading package from url {}", url);

    let mut response = get(url).expect("Failed to send request");
    let mut dest = File::create(output_path).expect("Failed to create file");
    copy(&mut response, &mut dest).expect("Failed to write to file");
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_download_remote() {
        let mut server = mockito::Server::new();

        let host = server.host_with_port();
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
        download_remote(&output_path, &url_str);

        assert!(output_path.is_file());

        mock.assert();
    }
}
