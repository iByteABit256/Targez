use std::path::PathBuf;
// use std::fs::File;
// use flate2::Compression;
// use flate2::write::GzEncoder;
use clap::{Parser, ValueEnum};

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
   file: Vec<PathBuf>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    /// Extract files from tar.gz file
    Extract,
    /// Compress files into tar.gz file
    Compress,
}

fn main() {
   let args = Args::parse();

   println!("Found {} input files!", args.file.into_iter().count());
   if let Some(target) = args.target {
        println!("Found target: {}", target);
   };

   match args.mode {
      Mode::Extract => println!("Extract mode"),
      Mode::Compress => println!("Compress mode")
   }

}
