pub use crate::cli::public::{Cli};

pub mod public {
    use clap::{arg, Parser};

    #[derive(Parser)]
    pub struct Cli {
        /// The pattern to look for
        pub pattern: String,
        /// The path to the file to read
        pub path: std::path::PathBuf,

        #[arg(short, long)]
        pub settings_file: Option<std::path::PathBuf>,

        #[arg(short, long)]
        pub debug: bool,
    }
    
    // TODO add parsing method here
}
