use reqwest::blocking::get;
use std::fs::File;
use std::io::copy;
use std::path::Path;

use crate::path::get_package_url;

/// Downloads a package from a remote URL and saves it to the specified output path.
///
/// # Arguments
///
/// * `output_path` - The path where the downloaded package will be saved.
/// * `package_name` - The name of the package to download.
/// * `version` - The version of the package to download.
///
/// # Errors
///
/// This function will panic if the request fails, if the file cannot be created,
/// or if writing to the file fails.
pub fn download_remote(output_path: &Path, package_name: &str, version: &str) {
    let url = get_package_url(package_name, version);
    println!("Downloading package from url {}", url);

    let mut response = get(&url).expect("Failed to send request");
    let mut dest = File::create(output_path).expect("Failed to create file");
    copy(&mut response, &mut dest).expect("Failed to write to file");
}
