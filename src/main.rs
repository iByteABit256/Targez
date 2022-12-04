use std::path::PathBuf;
use std::fs::File;
use std::io::ErrorKind;
use flate2::Compression;
use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use clap::{Parser, ValueEnum};
use walkdir::WalkDir;
use tar::Archive;

/// Compression/Archiving utility using tar and gzip
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Target file
   #[arg(short, long)]
   target: Option<String>,

   /// What mode to run the program in
   #[arg(value_enum)]
   mode: Mode,

   /// Input files/directories
   files: Vec<PathBuf>,
}

/// Targez Mode
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    /// Extract files from tar.gz file
    Extract,
    /// Compress files into tar.gz file
    Compress,
}

/// Compresses list of files into a tar.gz file and returns the total number of files compressed
fn compress(target: &str, input_files: Vec<PathBuf>) -> Result<usize, std::io::Error> {
   let tar_gz = File::create(target)?;
   let enc = GzEncoder::new(tar_gz, Compression::default());
   let mut tar = tar::Builder::new(enc);
   let mut file_count = 0;
   for file in input_files {
      let path_string = file.as_os_str();
      file_count = WalkDir::new(path_string).into_iter().count();

      if !file.exists() {
         return Err(std::io::Error::new(ErrorKind::NotFound, path_string.to_str().unwrap().to_owned() + " does not exist"));
      }

      if file.is_file() {
         tar.append_path_with_name(path_string, path_string)?;
      } else if file.is_dir() {
         tar.append_dir_all(path_string, path_string)?;
      } else {
         return Err(std::io::Error::new(ErrorKind::InvalidInput, path_string.to_str().unwrap().to_owned() + " is neither a file nor a directory"));
      }
   }
   Ok(file_count)
}

/// Extracts files from a tar.gz file and returns the total number of files extracted
fn extract(target: &str, input_files: Vec<PathBuf>) -> Result<usize, std::io::Error> {
   if input_files.len() != 1 {
      return Err(std::io::Error::new(ErrorKind::InvalidInput, "Exactly 1 file must be given for extraction"));
   }

   let path = input_files.into_iter().next().unwrap();

   let tar_gz = File::open(path)?;
   let tar = GzDecoder::new(tar_gz);
   let mut archive = Archive::new(tar);
   archive.unpack(target)?;

   let file_count = WalkDir::new(target).into_iter().count();

   Ok(file_count)
}

fn main() {
   let args = Args::parse();
   let files = args.files;

   println!("Found {} input files!", files.len());
   if let Some(target) = args.target {
      println!("Found target: {}", target);

      match args.mode {
         Mode::Extract => {
            let file_count = extract(&target, files).unwrap_or_else(|err| {
               eprintln!("Error during extraction: {err}");
               std::process::exit(1);
            });
            println!("Successfuly extracted {file_count} files into {target}!");
         },
         Mode::Compress => {
            let file_count = compress(&target, files).unwrap_or_else(|err| {
               eprintln!("Error during compression: {err}");
               std::process::exit(1);
            });
            println!("Successfuly compressed {file_count} files into {target}!");
         }
      }
   };
}
