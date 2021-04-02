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
        #[cfg(feature = "toml")]
        Some("toml") => toml::from_str(contents(path)?.as_str()).map_err(ConfigFileError::Toml),
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
    #[cfg(feature = "toml")]
    #[error("couldn't parse TOML file")]
    Toml(#[from] toml::de::Error),
    #[error("don't know how to parse file")]
    UnknownFormat,
}
