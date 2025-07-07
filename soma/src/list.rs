pub fn handle_list_command(json: bool, csv: bool, verbose: bool, noaction: bool) {
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
