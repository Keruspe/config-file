# config-file

[![API Docs](https://docs.rs/config-file/badge.svg)](https://docs.rs/config-file)
[![Downloads](https://img.shields.io/crates/d/config-file.svg)](https://crates.io/crates/config-file)

## Read and parse configuration file automatically

config-file reads your configuration files and parse them automatically using their extension.

## Features

- toml is enabled by default
- json is optional
- xml is optional
- yaml is optional

## Examples

```rust
use config_file::FromConfigFile;
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    host: String,
}

let config = Config::from_config_file("/etc/myconfig.toml").unwrap();
```
