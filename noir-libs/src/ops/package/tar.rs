use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::{self, BufWriter};
use std::path::Path;
use tar::Builder;

/// Compress and package a directory into a `.tar.gz` archive.
///
/// The resulting `.tar.gz` will contain the directory itself (not just its contents).
pub fn create_tar_gz(src_folder: &Path, dst_path: &Path) -> io::Result<()> {
    let tar_gz = File::create(dst_path)?;
    let enc_writer = BufWriter::new(tar_gz);
    let encoder = GzEncoder::new(enc_writer, Compression::default());

    let mut tar = Builder::new(encoder);

    tar.append_dir_all(src_folder.file_name().unwrap(), src_folder)?;

    let encoder = tar.into_inner()?;
    encoder.finish()?;
    Ok(())
}
