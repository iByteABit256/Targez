use std::path::PathBuf;

use clap::Parser;

/// Parses input for targez
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Target file
   #[arg(short, long)]
   target: Option<String>,

   /// Input files/directories
   file: Vec<PathBuf>,
}

fn main() {
   let args = Args::parse();

   println!("Found {} input files!", args.file.into_iter().count());
   if let Some(target) = args.target {
        println!("Found target: {}", target);
   };
}
