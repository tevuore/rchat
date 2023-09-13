#![allow(unused)]

use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,

    #[arg(short, long)]
    debug: bool,
}

fn main() {
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path).expect("could not read file");

    if args.debug {
        println!("DEBUG: Reading file {:?}", args.path);
        println!("DEBUG: Matching string {:?}", args.pattern);
    }
    let mut any_match = false;
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
            any_match = true;
        }
    }

    if !any_match {
        println!("No matches");
    }
}
