#![allow(unused)]

use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read, Result};
use std::path::Path;

use clap::{arg, Parser};
use toml::value::Table;

use crate::chatgpt_request::chatgpt_request;
use crate::cli::Cli;
use crate::debug_logger::{DebugLogger, EmptyDebugLogger, FileDebugLogger, StdoutDebugLogger};
use crate::settings::settings;

mod chatgpt_request;
mod cli;
mod debug_logger;
mod printer;
mod settings;

// TODO own error handling for file not found
// TODO can we have hierarchical error, what failed and why failed?
// TODO and nice cli output + stack trace with debug?

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    let log = build_debug_logger(&args);
    log.debug(&"Starting...");

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

    let settings = settings(&args, &log)?;

    // TODO rethink this way of passing logger, IoC way?

    printer::me_print_stdout(&prompt, &args);
    match chatgpt_request(&prompt, &settings.chatgpt, &log).await {
        Ok(response) => {
            printer::ai_print_stdout(&response, &args);
        }
        Err(e) => {
            printer::print_error(&e.to_string());
        }
    }

    log.debug(&"Exiting...");

    Ok(())
}

fn build_debug_logger(args: &Cli) -> Box<dyn DebugLogger> {
    if args.debug {
        if let Some(debug_file) = &args.debug_file {
            Box::new(FileDebugLogger::new(debug_file))
        } else {
            Box::new(StdoutDebugLogger)
        }
    } else {
        Box::new(EmptyDebugLogger)
    }
}
