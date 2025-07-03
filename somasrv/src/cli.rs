// use log::{debug, error, info, trace, warn};
// use simplelog::*;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = env!("CARGO_PKG_NAME"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    version = env!("CARGO_PKG_VERSION")
)]
pub struct Cli {
    /// Specify a configuration file
    #[structopt(short, long, parse(from_os_str))]
    pub config: Option<PathBuf>,

    /// Describe what would be done without doing it
    #[structopt(long, short)]
    pub validate: bool,

    /// Set the log level (error, warn, info, debug, trace)
    #[structopt(long, default_value = "info")]
    pub loglevel: String,
}

impl Cli {
    pub fn parse_args() -> Self {
        Cli::from_args()
    }

    // Open the configuration file
    // Set the logger to the specified level
    // Validate the config file if requested to do so and then exit
}
