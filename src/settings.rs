pub use public::*;

pub mod public {
    use super::private::*;
    use crate::cli::Cli;
    use std::io::Result;

    #[derive(Debug)]
    pub struct Settings {
        pub chatgpt: ChatGptSettings,
    }

    #[derive(Debug)]
    pub struct ChatGptSettings {
        pub api_key: String,
        pub model: String, // TODO should use enum?
    }

    pub fn settings(args: &Cli) -> Result<Settings> {
        _settings(args)
    }
}

mod private {
    use crate::cli::Cli;
    use crate::settings::{ChatGptSettings, Settings};
    use std::fs::File;
    use std::io::{BufReader, Read, Result};
    use std::path::Path;
    use toml::value::Table;

    use super::public;

    pub fn _settings(args: &Cli) -> Result<Settings> {
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
}
