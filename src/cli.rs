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

        #[arg(long)]
        pub debug_file: Option<std::path::PathBuf>, // TODO write test for command line as no two can be with -d

        #[arg(long)]
        pub no_markdown_output: bool,

        #[arg(long)]
        pub no_markdown_input: bool,

        #[arg(short, long)]
        pub custom_instructions: Option<String>,
    }

    pub fn parse() -> Cli {
        Cli::parse()
    }
}
