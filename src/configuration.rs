use serde_derive::{Deserialize, Serialize};
use toml;
use std::{fs, net::Ipv4Addr};
use chrono;

const CONFIGURATION_FILENAME: &str = "dhcp-server.toml";

#[derive(Deserialize, Serialize)]
pub struct Configuration {
    pub range: AddressRange,
}

#[derive(Deserialize, Serialize)]
pub struct AddressRange {
    pub start_address: Ipv4Addr,
    pub end_address: Ipv4Addr,
    pub subnet_mask: Ipv4Addr,
}

impl AddressRange {
    pub fn new(start_address: Ipv4Addr, end_address: Ipv4Addr, subnet_mask: Ipv4Addr) -> Self {
        AddressRange { start_address, end_address, subnet_mask }
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            range: AddressRange {
                start_address: [192,0,0,1].into(),
                end_address: [192,0,0,100].into(),
                subnet_mask: [255,255,255,0].into(),
            }
        }
    }
}
impl Configuration {
    pub fn new(range: AddressRange) -> Configuration {
        Configuration { range }
    }
    pub fn to_toml(&self) -> String {
        toml::to_string(&self).expect("Unable to parse configuration.")
    }
    pub fn from_toml(content: String) -> Configuration {
        toml::from_str(&content).expect("Unable to parse configuration.")
    }
}
