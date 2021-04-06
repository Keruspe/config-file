#![deny(missing_docs)]
#![warn(rust_2018_idioms)]
#![doc(html_root_url = "https://docs.rs/config-file/0.2.1/")]

//! # Read and parse configuration file automatically
//!
//! config-file reads your configuration files and parse them automatically using their extension.
//!
//! # Features
//!
//! - toml is enabled by default
//! - json is optional
//! - xml is optional
//! - yaml is optional
//!
//! # Examples
//!
//! ```rust,no_run
//! use config_file::FromConfigFile;
//! use serde::Deserialize;
//!
//! #[derive(Deserialize)]
//! struct Config {
//!     host: String,
//! }
//!
//! let config = Config::from_config_file("/etc/myconfig.toml").unwrap();
//! ```

use serde::de::DeserializeOwned;
use std::{ffi::OsStr, fs::File, path::Path};
use thiserror::Error;
#[cfg(feature = "toml")]
use toml_crate as toml;

/// Trait for loading a struct from a configuration file.
/// This trait is automatically implemented when serde::Deserialize is.
pub trait FromConfigFile {
    /// Load ourselves from the configuration file located at @path
    fn from_config_file<P: AsRef<Path>>(path: P) -> Result<Self, ConfigFileError>
    where
        Self: Sized;
}

impl<C: DeserializeOwned> FromConfigFile for C {
    fn from_config_file<P: AsRef<Path>>(path: P) -> Result<Self, ConfigFileError>
    where
        Self: Sized,
    {
        let path = path.as_ref();
        let extension = path
            .extension()
            .and_then(OsStr::to_str)
            .map(|extension| extension.to_lowercase());
        match extension.as_deref() {
            #[cfg(feature = "json")]
            Some("json") => {
                serde_json::from_reader(open_file(path)?).map_err(ConfigFileError::Json)
            }
            #[cfg(feature = "toml")]
            Some("toml") => toml::from_str(
                std::fs::read_to_string(path)
                    .map_err(ConfigFileError::FileAccess)?
                    .as_str(),
            )
            .map_err(ConfigFileError::Toml),
            #[cfg(feature = "xml")]
            Some("xml") => {
                serde_xml_rs::from_reader(open_file(path)?).map_err(ConfigFileError::Xml)
            }
            #[cfg(feature = "yaml")]
            Some("yaml") | Some("yml") => {
                serde_yaml::from_reader(open_file(path)?).map_err(ConfigFileError::Yaml)
            }
            _ => Err(ConfigFileError::UnsupportedFormat),
        }
    }
}

#[allow(unused)]
fn open_file(path: &Path) -> Result<File, ConfigFileError> {
    File::open(path).map_err(ConfigFileError::FileAccess)
}

/// This type represents all possible errors that can occur when loading data from a configuration file.
#[derive(Error, Debug)]
pub enum ConfigFileError {
    #[error("couldn't read config file")]
    /// There was an error while reading the configuration file
    FileAccess(#[from] std::io::Error),
    #[cfg(feature = "json")]
    #[error("couldn't parse JSON file")]
    /// There was an error while parsing the JSON data
    Json(#[from] serde_json::Error),
    #[cfg(feature = "toml")]
    #[error("couldn't parse TOML file")]
    /// There was an error while parsing the TOML data
    Toml(#[from] toml::de::Error),
    #[cfg(feature = "xml")]
    #[error("couldn't parse XML file")]
    /// There was an error while parsing the XML data
    Xml(#[from] serde_xml_rs::Error),
    #[cfg(feature = "yaml")]
    #[error("couldn't parse YAML file")]
    /// There was an error while parsing the YAML data
    Yaml(#[from] serde_yaml::Error),
    #[error("don't know how to parse file")]
    /// We don't know how to parse this format according to the file extension
    UnsupportedFormat,
}

#[cfg(test)]
mod test {
    use super::*;

    use serde::Deserialize;

    #[derive(Debug, Deserialize, PartialEq)]
    struct TestConfig {
        host: String,
        port: u64,
        tags: Vec<String>,
        inner: TestConfigInner,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct TestConfigInner {
        answer: u8,
    }

    impl TestConfig {
        #[allow(unused)]
        fn example() -> Self {
            Self {
                host: "example.com".to_string(),
                port: 443,
                tags: vec!["example".to_string(), "test".to_string()],
                inner: TestConfigInner { answer: 42 },
            }
        }
    }

    #[test]
    fn test_unknown() {
        let config = TestConfig::from_config_file("/tmp/foobar");
        assert!(matches!(config, Err(ConfigFileError::UnsupportedFormat)));
    }

    #[test]
    #[cfg(feature = "toml")]
    fn test_file_not_found() {
        let config = TestConfig::from_config_file("/tmp/foobar.toml");
        assert!(matches!(config, Err(ConfigFileError::FileAccess(_))));
    }

    #[test]
    #[cfg(feature = "json")]
    fn test_json() {
        let config = TestConfig::from_config_file("testdata/config.json");
        assert_eq!(config.unwrap(), TestConfig::example());
    }

    #[test]
    #[cfg(feature = "toml")]
    fn test_toml() {
        let config = TestConfig::from_config_file("testdata/config.toml");
        assert_eq!(config.unwrap(), TestConfig::example());
    }

    #[test]
    #[cfg(feature = "xml")]
    fn test_xml() {
        let config = TestConfig::from_config_file("testdata/config.xml");
        assert_eq!(config.unwrap(), TestConfig::example());
    }

    #[test]
    #[cfg(feature = "yaml")]
    fn test_yaml() {
        let config = TestConfig::from_config_file("testdata/config.yml");
        assert_eq!(config.unwrap(), TestConfig::example());
    }
}
