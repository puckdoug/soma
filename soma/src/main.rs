pub mod check;
pub mod cli;
pub mod list;
pub mod scan;

use check::handle_check_command;
use cli::{print_usage, Cli, Command};
use list::handle_list_command;
use scan::handle_scan_command;

fn main() {
    // Parse command line arguments
    let cli = Cli::parse_args();

    if cli.verbose {
        println!("Running in verbose mode");
    }

    if cli.noaction {
        println!("Running in no-action mode (dry run)");
    }

    match &cli.command {
        Some(Command::Help) => {
            print_usage();
        }
        Some(Command::List { json, csv }) => {
            handle_list_command(*json, *csv, cli.verbose, cli.noaction);
        }
        Some(Command::Scan { json, csv }) => {
            handle_scan_command(*json, *csv, cli.verbose, cli.noaction);
        }
        Some(Command::Check { json, csv, hosts }) => {
            handle_check_command(*json, *csv, hosts, cli.verbose, cli.noaction);
        }
        None => {
            println!("No subcommand specified. Use 'soma help' for usage information.");
            println!("{:?}", cli);
        }
    }
}
