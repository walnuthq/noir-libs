use std::{fs, path::Path, process::Command};

use crate::path::get_package_url;

pub fn download_remote(output_path: &Path, package_name: &str, version: &str) {
    let url = get_package_url(package_name, version);

    let mut curl = Command::new("curl");
    println!("Downloading from url {}", url);
    //    curl.args(["--proto", "=https", "--tlsv1.2", "-fL"]);
    curl.args(["--proto", "=http", "--tlsv1.2", "-fL"]);
    curl.arg("-o");
    curl.arg(output_path);
    curl.arg(&url);
    eprintln!("{curl:?}");
    let curl_exit = curl.status().expect("Failed to start curl");
    if !curl_exit.success() {
        panic!("Failed to download {url} with curl")
    }
}
