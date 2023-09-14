#![allow(unused)]

use clap::{arg, Parser};
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read, Result};
use std::path::Path;
use toml::value::Table;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,

    #[arg(short, long)]
    settings_file: Option<std::path::PathBuf>,

    #[arg(short, long)]
    debug: bool,
}

// TODO own error handling for file not found
// TODO can we have hierarchical error, what failed and why failed?
// TODO and nice cli output + stack trace with debug?

fn main() -> std::io::Result<()> {
    let args = Cli::parse();

    if args.debug {
        println!("DEBUG: Reading file {:?}", args.path);
        println!("DEBUG: Matching string {:?}", args.pattern);
    }
    let settings = settings(&args)?;

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

fn settings(args: &Cli) -> Result<Settings> {
    // TODO could read setting file location from env
    // TODO could read a setting from env

    // TODO debug if file exists
    let settings = match &args.settings_file {
        Some(path) => {
            if path.exists() {
                if args.debug {
                    println!("DEBUG: Reading settings file {:?}", path);
                }
                read_settings(&path)?
            } else {
                if args.debug {
                    println!(
                        "DEBUG: Settings file {:?} does not exist, using default",
                        path
                    );
                }
                Settings {
                    chatgpt: default_chatgpt_settings(),
                }
            }
        } // TODO test error

        // TODO you should be able to run rchat without chatgpt config
        // if operation doesn't require prompting to chatgpt
        // TODO options validation command
        None => default_settings(),
    };

    Ok(settings)
}

fn read_settings(path: &Path) -> std::io::Result<Settings> {
    let f = File::open(path)?;
    let mut reader = BufReader::new(f);

    let data = std::fs::read_to_string(&path)?;

    let value = match data.parse::<Table>() {
        Ok(value) => value,
        Err(e) => {
            println!("Error parsing settings file: {}", e);
            std::process::exit(1);
        }
    };

    Ok(Settings {
        // TODO could use toml deserialize
        chatgpt: ChatGptSettings {
            api_key: value["chatgpt"]["api_key"].as_str().unwrap().to_string(), // TODO what happens if key doesn't exist? TEST
            model: value["chatgpt"]["model"].as_str().unwrap().to_string(),
        },
    })
}

fn default_settings() -> Settings {
    Settings {
        chatgpt: default_chatgpt_settings(),
    }
}

fn default_chatgpt_settings() -> ChatGptSettings {
    ChatGptSettings {
        api_key: "undefined".to_string(),
        model: "undefined".to_string(),
    }
}

#[derive(Debug)]
struct Settings {
    chatgpt: ChatGptSettings,
}

#[derive(Debug)]
struct ChatGptSettings {
    api_key: String,
    model: String,
}
