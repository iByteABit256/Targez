mod parser;
mod targez;

use parser::{Args, Mode};
use clap::Parser;
use targez::{compress, extract};


fn main() {
   let args = Args::parse();
   let files = args.files;
   let exclude_vcs = args.exclude_vcs;

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
            let file_count = compress(&target, files, exclude_vcs).unwrap_or_else(|err| {
               eprintln!("Error during compression: {err}");
               std::process::exit(1);
            });
            println!("Successfuly compressed {file_count} files into {target}!");
         }
      }
   };
}
