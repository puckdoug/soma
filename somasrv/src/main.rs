pub mod cli;
pub mod config;

use cli::Cli;
use config::Config;

fn main() {
    // Parse command line arguments
    let cli = Cli::parse_args();
    let cfg = Config::from_file(&cli.config);

    println!("{:?}", cli);
    println!("{:?}", cfg);
}
