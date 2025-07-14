// use serde::{Deserialize, Serialize};
use serde_json;
use somacommon::Host;

pub fn handle_list_command(json: bool, csv: bool, verbose: bool, noaction: bool) {
    if verbose {
        println!("Executing list command");
    }

    if noaction {
        println!("Would list all managed hosts");
        return;
    }

    let hosts: Vec<Host> = Vec::new();

    if json {
        let hostlist = serde_json::to_string_pretty(&hosts).unwrap();
        println!("{hostlist}");
    } else if csv {
        println!("hostname");
        for hostname in hosts {
            println!("{hostname}");
        }
    } else {
        println!("hostname");
        for hostname in hosts {
            println!("{hostname}");
        }
    }
}
