#![allow(unused)]

use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};

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

// TODO own error handling for file not found
fn main() -> std::io::Result<()> {
    let args = Cli::parse();

    if args.debug {
        println!("DEBUG: Reading file {:?}", args.path);
        println!("DEBUG: Matching string {:?}", args.pattern);
    }
    //let content = std::fs::read_to_string(&args.path).expect("could not read file");

    let f = File::open(&args.path)?;
    let mut reader = BufReader::new(f);

    let mut any_match = false;
    let mut line = String::new();
    while let Ok(len) = reader.read_line(&mut line) {
        if len == 0 {
            break;
        }
        if args.debug {
            println!("DEBUG read {} bytes", len);
        }
        if line.contains(&args.pattern) {
            println!("{}", line);
            any_match = true;
        }

        // reset buffer
        line.truncate(0);
    }

    if !any_match {
        println!("No matches");
    }

    Ok(())
}
