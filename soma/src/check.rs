pub fn handle_check_command(
    json: bool,
    csv: bool,
    hosts: &[String],
    verbose: bool,
    noaction: bool,
) {
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
