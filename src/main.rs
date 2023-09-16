#![allow(unused)]

mod cli;
use crate::cli::Cli;
mod chatgpt_request;
mod settings;
use crate::chatgpt_request::chatgpt_request;

use crate::settings::settings;

use clap::{arg, Parser};
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read, Result};
use std::path::Path;
use toml::value::Table;

// TODO own error handling for file not found
// TODO can we have hierarchical error, what failed and why failed?
// TODO and nice cli output + stack trace with debug?

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    if args.debug {
        println!("DEBUG: Reading file {:?}", args.path);
        println!("DEBUG: Matching string {:?}", args.pattern);
    }
    let settings = settings(&args)?;

    chatgpt_request(&settings.chatgpt).await;
    println!("Done");

    // let f = File::open(&args.path)?;
    // let mut reader = BufReader::new(f);
    //
    // let mut any_match = false;
    // let mut line = String::new();
    // while let Ok(len) = reader.read_line(&mut line) {
    //     if len == 0 {
    //         break;
    //     }
    //     if args.debug {
    //         println!("DEBUG read {} bytes", len);
    //     }
    //     if line.contains(&args.pattern) {
    //         println!("{}", line);
    //         any_match = true;
    //     }
    //
    //     // reset buffer
    //     line.truncate(0);
    // }
    //
    // if !any_match {
    //     println!("No matches");
    // }

    Ok(())
}
