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
    use dirs::home_dir;
    use std::fs::File;
    use std::io::{BufReader, Read, Result};
    use std::path::{Path, PathBuf};
    use toml::value::Table;

    use super::public;

    // TODO how to validate model is correct one (name)?

    pub fn _settings(args: &Cli) -> Result<Settings> {
        // TODO could read setting file location from env
        // TODO could read a setting from env
        let default_settings_file = default_home_settings_file();

        let settings_file = args
            .settings_file
            .as_ref()
            .or_else(|| Some(&default_settings_file));

        // TODO debug if file exists

        // TODO fix this as match is not needed? Question is how settings are layered, i.e. if given
        // specific file doesn't exists but default one exists? And then if both are missing then return plain defaults?
        let settings = match &settings_file {
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

    fn get_file_in_home_dir(filename: &str) -> Option<PathBuf> {
        home_dir().map(|mut path| {
            path.push(filename);
            path
        })
    }

    fn default_home_settings_file() -> PathBuf {
        get_file_in_home_dir(".rchat.toml").unwrap()
    }
    // fn default_home_settings_file() -> PathBuf {
    //     if let Some(mut path) = home_dir() {
    //         path.push("your_file.txt");
    //         println!("File path: {:?}", path);
    //     } else {
    //         println!("Home directory not found");
    //     }
    //     Path::new("~/.rchat.toml").to_path_buf()
    // }

    fn read_settings(path: &Path) -> Result<Settings> {
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
