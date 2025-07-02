pub mod cli;

use cli::Cli;

fn main() {
    // Parse command line arguments
    let cli = Cli::parse_args();

    println!("{:?}", cli);
}
