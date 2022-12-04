use std::path::PathBuf;
use clap::{Parser, ValueEnum};

/// Compression/Archiving utility using tar and gzip
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
   /// Target file
   #[arg(short, long)]
   pub target: Option<String>,

   /// What mode to run the program in
   #[arg(value_enum)]
   pub mode: Mode,

   /// Input files/directories
   pub files: Vec<PathBuf>,
}

/// Targez Mode
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Mode {
    /// Extract files from tar.gz file
    Extract,
    /// Compress files into tar.gz file
    Compress,
}
