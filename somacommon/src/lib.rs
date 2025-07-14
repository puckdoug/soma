use serde_derive::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct Host {
    hostname: String,
}

impl fmt::Display for Host {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.hostname)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_of_hosts() {
        let mut hosts: Vec<Host> = Vec::new();
        hosts.push(Host {
            hostname: "example.com".to_string(),
        });
        assert!(hosts.len() == 1);
        assert!(hosts[0].hostname == "example.com");
    }
}
