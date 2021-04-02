use serde::de::DeserializeOwned;
use std::{ffi::OsStr, path::Path};
use thiserror::Error;
#[cfg(feature = "toml")]
use toml_crate as toml;

pub type Result<T> = std::result::Result<T, ConfigFileError>;

pub fn parse<C: DeserializeOwned, P: AsRef<Path>>(path: P) -> Result<C> {
    let path = path.as_ref();
    let extension = path.extension().and_then(OsStr::to_str);
    match extension {
        #[cfg(feature = "json")]
        Some("json") => serde_json::from_str(contents(path)?.as_str()).map_err(ConfigFileError::Json),
        #[cfg(feature = "toml")]
        Some("toml") => toml::from_str(contents(path)?.as_str()).map_err(ConfigFileError::Toml),
        #[cfg(feature = "yaml")]
        Some("yaml")|Some("yml") => serde_yaml::from_str(contents(path)?.as_str()).map_err(ConfigFileError::Yaml),
        _ => Err(ConfigFileError::UnknownFormat),
    }
}

#[allow(unused)]
fn contents(path: &Path) -> Result<String> {
    std::fs::read_to_string(path).map_err(ConfigFileError::FileRead)
}

#[derive(Error, Debug)]
pub enum ConfigFileError {
    #[error("couldn't read config file")]
    FileRead(#[from] std::io::Error),
    #[cfg(feature = "json")]
    #[error("couldn't parse JSON file")]
    Json(#[from] serde_json::Error),
    #[cfg(feature = "toml")]
    #[error("couldn't parse TOML file")]
    Toml(#[from] toml::de::Error),
    #[cfg(feature = "yaml")]
    #[error("couldn't parse YAML file")]
    Yaml(#[from] serde_yaml::Error),
    #[error("don't know how to parse file")]
    UnknownFormat,
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
        fn example() -> Self {
            Self {
                host: "example.com".to_string(),
                port: 443,
                tags: vec!["example".to_string(), "test".to_string()],
                inner: TestConfigInner {
                    answer: 42,
                },
            }
        }
    }

    #[test]
    fn test_unknown() {
        let config = parse::<TestConfig, _>("/tmp/foobar");
        assert!(matches!(config, Err(ConfigFileError::UnknownFormat)));
    }

    #[test]
    #[cfg(feature = "toml")]
    fn test_file_not_found() {
        let config = parse::<TestConfig, _>("/tmp/foobar.toml");
        assert!(matches!(config, Err(ConfigFileError::FileRead(_))));
    }

    #[test]
    #[cfg(feature = "json")]
    fn test_json() {
        let config = parse::<TestConfig, _>("testdata/config.json");
        assert_eq!(config.unwrap(), TestConfig::example());
    }

    #[test]
    #[cfg(feature = "toml")]
    fn test_toml() {
        let config = parse::<TestConfig, _>("testdata/config.toml");
        assert_eq!(config.unwrap(), TestConfig::example());
    }

    #[test]
    #[cfg(feature = "yaml")]
    fn test_yaml() {
        let config = parse::<TestConfig, _>("testdata/config.yml");
        assert_eq!(config.unwrap(), TestConfig::example());
    }
}
