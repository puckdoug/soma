use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

const DEFAULT_CFG_PATH: &str = "/etc/soma.toml";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub logfile: PathBuf,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            logfile: PathBuf::from("/var/log/somasrv.log"),
        }
    }
}

impl Config {
    pub fn from_file(pb: &Option<PathBuf>) -> Result<Self, ConfigError> {
        let path: &Path;
        match pb {
            Some(p) => path = p.as_ref(),
            None => path = &Path::new(DEFAULT_CFG_PATH),
        }

        if !path.exists() {
            return Err(ConfigError::FileNotFound(path.to_path_buf()));
        }

        let content =
            fs::read_to_string(path).map_err(|e| ConfigError::ReadError(path.to_path_buf(), e))?;

        let config: Config =
            toml::from_str(&content).map_err(|e| ConfigError::ParseError(path.to_path_buf(), e))?;

        config.validate()?;
        Ok(config)
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<(), ConfigError> {
        Ok(())
    }
}

/// Configuration error types
#[derive(Debug)]
pub enum ConfigError {
    FileNotFound(PathBuf),
    ReadError(PathBuf, std::io::Error),
    ParseError(PathBuf, toml::de::Error),
    WriteError(PathBuf, std::io::Error),
    InvalidLogLevel(String),
    WebrootNotDirectory(PathBuf),
    LogDirectoryNotDirectory(PathBuf),
    AlreadyInitialized,
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::FileNotFound(path) => {
                write!(f, "Configuration file not found: {}", path.display())
            }
            ConfigError::ReadError(path, err) => {
                write!(
                    f,
                    "Error reading configuration file {}: {}",
                    path.display(),
                    err
                )
            }
            ConfigError::ParseError(path, err) => {
                write!(
                    f,
                    "Error parsing configuration file {}: {}",
                    path.display(),
                    err
                )
            }
            ConfigError::WriteError(path, err) => {
                write!(
                    f,
                    "Error writing configuration file {}: {}",
                    path.display(),
                    err
                )
            }
            ConfigError::InvalidLogLevel(level) => {
                write!(f, "Invalid log level: {}", level)
            }
            ConfigError::WebrootNotDirectory(path) => {
                write!(f, "Webroot path is not a directory: {}", path.display())
            }
            ConfigError::LogDirectoryNotDirectory(path) => {
                write!(
                    f,
                    "Log directory path is not a directory: {}",
                    path.display()
                )
            }
            ConfigError::AlreadyInitialized => {
                write!(f, "SomaSrv is already initialized")
            }
        }
    }
}

impl std::error::Error for ConfigError {}
