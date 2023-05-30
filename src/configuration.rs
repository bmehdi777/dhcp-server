use serde_derive::{Deserialize, Serialize};
use toml;
use std::fs;
use chrono;

const CONFIGURATION_FILENAME: &str = "dhcp-server.toml";

#[derive(Deserialize, Serialize)]
pub struct Configuration {
    range: AddressRange,
}

#[derive(Deserialize, Serialize)]
pub struct AddressRange {
    start_address: [u8; 4],
    end_address: [u8; 4],
    subnet_mask: [u8; 4],
}

impl AddressRange {
    pub fn new(start_address: [u8; 4], end_address: [u8; 4], subnet_mask: [u8; 4]) -> AddressRange {
        AddressRange { start_address, end_address, subnet_mask }
    }
}

impl Configuration {
    pub fn new(range: AddressRange) -> Configuration {
        Configuration { range }
    }
    pub fn default() -> Configuration {
        Configuration {
            range: AddressRange {
                start_address: [192,0,0,1],
                end_address: [192,0,0,100],
                subnet_mask: [255,255,255,0],
            }
        }
    }
    pub fn to_toml(&self) -> String {
        toml::to_string(&self).expect("Unable to parse configuration.")
    }
    pub fn from_toml(content: String) -> Configuration {
        toml::from_str(&content).expect("Unable to parse configuration.")
    }
}
