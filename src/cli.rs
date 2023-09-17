pub use crate::cli::public::Cli;

pub mod public {
    use clap::{arg, Parser};

    #[derive(Parser)]
    pub struct Cli {
        /// prompt
        pub prompt: Option<String>,

        #[arg(short, long)]
        pub settings_file: Option<std::path::PathBuf>,

        #[arg(short, long)]
        pub debug: bool,
    }

    // TODO add parsing method here
}
