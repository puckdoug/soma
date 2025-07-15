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

    /// Set the IP and optionally port to listen on
    #[structopt(long, short)]
    pub listen: String,

    /// Specify port to listen on
    #[structopt(long, short = "p")]
    pub port: Option<u16>,

    /// Set the log level (error, warn, info, debug, trace)
    #[structopt(long, default_value = "info")]
    pub loglevel: String,
}

impl Cli {
    pub fn parse_args() -> Self {
        let cli = Cli::from_args();
        cli.validate().unwrap_or_else(|e| {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        });
        cli
    }

    /// Validate the CLI arguments
    pub fn validate(&self) -> Result<(), String> {
        // Check if both listen contains a port and --port is specified
        if let Some(_port) = self.port {
            if self.listen.contains(':') {
                return Err(
                    "Cannot specify both a port in --listen and --port option. Use only one."
                        .to_string(),
                );
            }
        }

        // Validate listen address format
        self.validate_listen_address()?;

        Ok(())
    }

    /// Validate the listen address format
    fn validate_listen_address(&self) -> Result<(), String> {
        if self.listen.contains(':') {
            // Contains port, validate IP:PORT format
            let parts: Vec<&str> = self.listen.rsplitn(2, ':').collect();
            if parts.len() != 2 {
                return Err(format!("Invalid listen address format: {}", self.listen));
            }

            let port_str = parts[0];
            let ip_str = parts[1];

            // Validate port
            match port_str.parse::<u16>() {
                Ok(port) => {
                    if port == 0 {
                        return Err("Port cannot be 0".to_string());
                    }
                }
                Err(_) => return Err(format!("Invalid port number: {}", port_str)),
            }

            // Validate IP address
            if !self.is_valid_ip(ip_str) {
                return Err(format!("Invalid IP address: {}", ip_str));
            }
        } else {
            // No port specified, just validate IP
            if !self.is_valid_ip(&self.listen) {
                return Err(format!("Invalid IP address: {}", self.listen));
            }
        }

        Ok(())
    }

    /// Check if a string is a valid IPv4 or IPv6 address
    fn is_valid_ip(&self, ip: &str) -> bool {
        ip.parse::<std::net::IpAddr>().is_ok()
    }

    // Open the configuration file
    // Set the logger to the specified level
    // Validate the config file if requested to do so and then exit
}
