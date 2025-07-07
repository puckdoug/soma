pub mod cli;

use cli::{Cli, Command};

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

fn print_usage() {
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

fn handle_list_command(json: bool, csv: bool, verbose: bool, noaction: bool) {
    if verbose {
        println!("Executing list command");
    }

    if noaction {
        println!("Would list all managed hosts");
        return;
    }

    // Mock data for demonstration
    let hosts = vec![
        ("host1.example.com", "192.168.1.10", "managed"),
        ("host2.example.com", "192.168.1.11", "managed"),
        ("host3.example.com", "192.168.1.12", "managed"),
    ];

    if json {
        println!("{{");
        println!("  \"hosts\": [");
        for (i, (hostname, ip, status)) in hosts.iter().enumerate() {
            let comma = if i == hosts.len() - 1 { "" } else { "," };
            println!("    {{");
            println!("      \"hostname\": \"{}\",", hostname);
            println!("      \"ip\": \"{}\",", ip);
            println!("      \"status\": \"{}\"", status);
            println!("    }}{}", comma);
        }
        println!("  ]");
        println!("}}");
    } else if csv {
        println!("hostname,ip,status");
        for (hostname, ip, status) in hosts {
            println!("{},{},{}", hostname, ip, status);
        }
    } else {
        println!("Managed Hosts:");
        println!("{:<20} {:<15} {:<10}", "Hostname", "IP Address", "Status");
        println!("{:-<20} {:-<15} {:-<10}", "", "", "");
        for (hostname, ip, status) in hosts {
            println!("{:<20} {:<15} {:<10}", hostname, ip, status);
        }
    }
}

fn handle_scan_command(json: bool, csv: bool, verbose: bool, noaction: bool) {
    if verbose {
        println!("Executing scan command");
    }

    if noaction {
        println!("Would scan network for managed and unmanaged hosts");
        return;
    }

    // Mock data for demonstration
    let hosts = vec![
        ("host1.example.com", "192.168.1.10", "managed"),
        ("host2.example.com", "192.168.1.11", "managed"),
        ("unknown-device", "192.168.1.15", "unmanaged"),
        ("printer", "192.168.1.20", "unmanaged"),
    ];

    if json {
        println!("{{");
        println!("  \"scan_results\": [");
        for (i, (hostname, ip, status)) in hosts.iter().enumerate() {
            let comma = if i == hosts.len() - 1 { "" } else { "," };
            println!("    {{");
            println!("      \"hostname\": \"{}\",", hostname);
            println!("      \"ip\": \"{}\",", ip);
            println!("      \"status\": \"{}\"", status);
            println!("    }}{}", comma);
        }
        println!("  ]");
        println!("}}");
    } else if csv {
        println!("hostname,ip,status");
        for (hostname, ip, status) in hosts {
            println!("{},{},{}", hostname, ip, status);
        }
    } else {
        println!("Network Scan Results:");
        println!("{:<20} {:<15} {:<10}", "Hostname", "IP Address", "Status");
        println!("{:-<20} {:-<15} {:-<10}", "", "", "");
        for (hostname, ip, status) in hosts {
            println!("{:<20} {:<15} {:<10}", hostname, ip, status);
        }
    }
}

fn handle_check_command(json: bool, csv: bool, hosts: &[String], verbose: bool, noaction: bool) {
    if verbose {
        println!("Executing check command");
    }

    let target_hosts = if hosts.is_empty() {
        vec![
            "host1.example.com",
            "host2.example.com",
            "host3.example.com",
        ]
    } else {
        hosts.iter().map(|s| s.as_str()).collect()
    };

    if noaction {
        println!("Would check status of hosts: {:?}", target_hosts);
        return;
    }

    // Mock status data
    let status_data: Vec<(&str, &str, &str, &str)> = target_hosts
        .iter()
        .map(|host| (*host, "online", "healthy", "last_seen: 2024-01-01 12:00:00"))
        .collect();

    if json {
        println!("{{");
        println!("  \"status_reports\": [");
        for (i, (hostname, status, health, last_seen)) in status_data.iter().enumerate() {
            let comma = if i == status_data.len() - 1 { "" } else { "," };
            println!("    {{");
            println!("      \"hostname\": \"{}\",", hostname);
            println!("      \"status\": \"{}\",", status);
            println!("      \"health\": \"{}\",", health);
            println!("      \"last_seen\": \"{}\"", last_seen);
            println!("    }}{}", comma);
        }
        println!("  ]");
        println!("}}");
    } else if csv {
        println!("hostname,status,health,last_seen");
        for (hostname, status, health, last_seen) in status_data {
            println!("{},{},{},{}", hostname, status, health, last_seen);
        }
    } else {
        println!("Host Status Reports:");
        println!(
            "{:<20} {:<10} {:<10} {:<25}",
            "Hostname", "Status", "Health", "Last Seen"
        );
        println!("{:-<20} {:-<10} {:-<10} {:-<25}", "", "", "", "");
        for (hostname, status, health, last_seen) in status_data {
            println!(
                "{:<20} {:<10} {:<10} {:<25}",
                hostname, status, health, last_seen
            );
        }
    }
}
