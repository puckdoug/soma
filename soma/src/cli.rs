use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = env!("CARGO_PKG_NAME"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    version = env!("CARGO_PKG_VERSION")
)]
pub struct Cli {
    /// Describe what would be done without doing it
    #[structopt(long, short)]
    pub noaction: bool,

    /// Produce verbose output as the process runs
    #[structopt(long, short)]
    pub verbose: bool,
}

impl Cli {
    pub fn parse_args() -> Self {
        Cli::from_args()
    }
}
