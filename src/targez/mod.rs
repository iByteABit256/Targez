use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use tar::Archive;
use walkdir::WalkDir;

/// Compresses list of files into a tar.gz file and returns the total number of files compressed
pub fn compress(target: &str, input_files: Vec<PathBuf>, exclude_vcs: bool) -> Result<usize, Error> {
    let tar_gz = File::create(target)?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    let mut file_count = 0;

    for file in input_files {
        let path_string = file.clone().into_os_string().into_string().unwrap();
        file_count = WalkDir::new(&path_string).into_iter().count();

        if !file.exists() {
            return Err(Error::new(ErrorKind::NotFound,format!("{} does not exist", &path_string)));
        }

        if file.is_file() {
            tar.append_path(&path_string, exclude_vcs)?;
        } else if file.is_dir() {
            tar.append_dir_all(&path_string, &path_string, exclude_vcs)?;
        } else {
            return Err(Error::new(ErrorKind::InvalidInput,format!("{} is neither a file nor a directory", &path_string)));
        }
    }
    Ok(file_count)
}

/// Extracts files from a tar.gz file and returns the total number of files extracted
pub fn extract(target: &str, input_files: Vec<PathBuf>) -> Result<usize, Error> {
    if input_files.len() != 1 {
        return Err(Error::new(ErrorKind::InvalidInput,"Exactly 1 file must be given for extraction",));
    }

    let path = input_files.into_iter().next().unwrap();

    let tar_gz = File::open(path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(target)?;

    let file_count = WalkDir::new(target).into_iter().count();

    Ok(file_count)
}
