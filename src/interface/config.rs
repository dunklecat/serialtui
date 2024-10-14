use std::fs;

use serde::Deserialize;
use serde::Serialize;
use serde_enum_str::Deserialize_enum_str;
use serde_enum_str::Serialize_enum_str;

use super::raw_protocol::RawProtocol;
use super::raw_serial::RawSerial;

#[derive(
    Default, Debug, Clone, Deserialize_enum_str, Serialize_enum_str, enum_iterator::Sequence,
)]
pub enum Mode {
    BINARY,
    JSON,
    HEX,
    #[default]
    HUMAN,
}

#[derive(Default, Deserialize, Serialize)]
pub struct Config {
    #[serde(default = "RawProtocol::default")]
    pub protocol: RawProtocol,
    #[serde(default = "RawSerial::default")]
    pub serial: RawSerial,
    #[serde(default = "Mode::default")]
    pub mode: Mode,
}

#[derive(Default)]
pub struct ConfigBuilder {
    config_path: String,
}

impl ConfigBuilder {
    pub fn new() -> ConfigBuilder {
        ConfigBuilder {
            config_path: Default::default(),
        }
    }

    pub fn config_path(mut self, config_path: &str) -> ConfigBuilder {
        self.config_path = String::from(config_path);
        self
    }

    pub fn build(self) -> Config {
        toml::from_str(
            fs::read_to_string(self.config_path)
                .expect("Should have been able to read the file")
                .as_str(),
        )
        .unwrap()
    }
}
