use std::{fs, path::Path, process::Command};


pub fn download_remote(output_path: &Path, package_name: &str, version: &str) {
//let url = "https://ucf6a2a9faa54417391e14393456.dl.dropboxusercontent.com/cd/0/inline2/Cf9NALRz-tLZOpKijCKfJD-MVhCr7_IhfLRYfXolteRUtsBmL9e-ReHx9fbFeVIJdDGtIQdBgSoRKAO1BwAYb-dBLvy2tnFnUFwTh7pGolActziBmwDEAUoIqkoQhYsM6QLhAjogIKt328LoH-EDHKsdH26nTtq3kuN_cge6fFYx40PyeViQ25u6iLlCgUMtLE0CDz3Rx08-42XaUWK-GkomC2tU8xvKK7thvxJKw3Du3UANQj_viSJCxRZzRVfwK4TOh0rAGvk9d3gs_Bqv60A9Bm3mUqW2tgSFF9vQThVBwGuNB8xMPzYlFoKDVMkycIDP20x-u0Q0mGg8So-GIdAU52pmydFQWr0stmMv5PcaWA/file";
    let filename = format!("{}-{}", package_name, version);
    let url = format!("http://127.0.0.1:8888/{}/{}/{}", package_name, version, filename);
//    let url: String = format!("https://github.com/starkware-libs/cairo/archive/{rev}.zip");
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