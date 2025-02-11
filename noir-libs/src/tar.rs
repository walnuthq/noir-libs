use std::fs;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::{self, BufWriter};
use std::path::Path;
use flate2::read::GzDecoder;
use tar::{Archive, Builder};

/// Compress and package a directory into a `.tar.gz` archive.
///
/// The resulting `.tar.gz` will contain the directory itself (not just its contents).
pub fn create_tar_gz(src_folder: &Path, dst_path: &Path) -> io::Result<()> {
    let tar_gz = File::create(dst_path)?;
    let enc_writer = BufWriter::new(tar_gz);
    let encoder = GzEncoder::new(enc_writer, Compression::default());

    let mut tar = Builder::new(encoder);

    // package all files inside src_folder
    for entry in fs::read_dir(src_folder)? {
        let entry = entry?;
        let path = entry.path();
        let relative_path = path.strip_prefix(src_folder).unwrap();

        if path.is_dir() {
            tar.append_dir_all(relative_path, &path)?;
        } else {
            let mut file = File::open(&path)?;
            tar.append_file(relative_path, &mut file)?;
        }
    }

    let encoder = tar.into_inner()?;
    encoder.finish()?;
    Ok(())
}

/// Extracts a `.tar.gz` archive to the specified destination folder.
///
/// The archive must contain a directory, which will be extracted into `dst_folder`.
pub fn extract_tar_gz(archive_path: &Path, dst_folder: &Path) -> io::Result<()> {
    let tar_gz = File::open(archive_path)?;
    let decoder = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(decoder);

    archive.unpack(dst_folder)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use tempfile::tempdir;
    use crate::config::MANIFEST_FILE_NAME;
    use crate::tar::extract_tar_gz;

    const TEST_PACKAGE: &str = "tests/test_files/test_package-1.2.3";

    #[test]
    fn test_extract_package() {
        let temp_dir = tempdir().unwrap();
        let package_path = Path::new(TEST_PACKAGE);

        let result = extract_tar_gz(package_path, temp_dir.path());
        assert!(result.is_ok());
        assert!(temp_dir.path().join(MANIFEST_FILE_NAME).exists()); // Extracted files should include manifest
    }
}