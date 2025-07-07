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

    #[structopt(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    /// Print out the usage information
    Help,
    /// List all hosts that can be managed
    List {
        /// Return the information in JSON format
        #[structopt(long)]
        json: bool,
        /// Return the information in CSV format
        #[structopt(long)]
        csv: bool,
    },
    /// Scan the network for hosts both managed and unmanaged
    Scan {
        /// Return the information in JSON format
        #[structopt(long)]
        json: bool,
        /// Return the information in CSV format
        #[structopt(long)]
        csv: bool,
    },
    /// Request a status report from a host or list of hosts or all hosts if none are specified
    Check {
        /// Return the information in JSON format
        #[structopt(long)]
        json: bool,
        /// Return the information in CSV format
        #[structopt(long)]
        csv: bool,
        /// List of hosts to check (if none specified, check all hosts)
        #[structopt()]
        hosts: Vec<String>,
    },
}

impl Cli {
    pub fn parse_args() -> Self {
        Cli::from_args()
    }
}

pub fn print_usage() {
    println!("SOMA - System Operations Management Agent");
    println!();
    println!("USAGE:");
    println!("    soma [FLAGS] <SUBCOMMAND>");
    println!();
    println!("FLAGS:");
    println!("    -n, --noaction    Describe what would be done without doing it");
    println!("    -v, --verbose     Produce verbose output as the process runs");
    println!("    -h, --help        Prints help information");
    println!("    -V, --version     Prints version information");
    println!();
    println!("SUBCOMMANDS:");
    println!("    help     Print out the usage information");
    println!("    list     List all hosts that can be managed");
    println!("    scan     Scan the network for hosts both managed and unmanaged");
    println!("    check    Request a status report from a host or list of hosts");
    println!();
    println!("Each subcommand supports:");
    println!("    --json    Return information in JSON format");
    println!("    --csv     Return information in CSV format");
}
