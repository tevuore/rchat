#![allow(unused)]

use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read, Result};
use std::path::Path;

use clap::{arg, Parser};
use toml::value::Table;

use crate::chatgpt_request::chatgpt_request;
use crate::cli::Cli;
use crate::settings::settings;

mod cli;

mod chatgpt_request;
mod settings;

// TODO own error handling for file not found
// TODO can we have hierarchical error, what failed and why failed?
// TODO and nice cli output + stack trace with debug?

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    let prompt = match &args.prompt {
        Some(prompt) => prompt.clone(),
        None => {
            let mut input = String::new();
            println!("Enter prompt:");
            std::io::stdin().read_line(&mut input)?;
            let stdin_prompt = input.trim().to_string();

            stdin_prompt
        }
    };

    let settings = settings(&args)?;

    chatgpt_request(&prompt, &settings.chatgpt).await;
    println!("Done");

    Ok(())
}
