use chrono;
use serde_derive::{Deserialize, Serialize};
use std::{fs, net::Ipv4Addr};

const CONFIGURATION_FILENAME: &str = "dhcp-server.toml";

#[derive(Deserialize, Serialize)]
pub struct Configuration {
    pub range: AddressRange,
    pub configuration_path: String,
}

#[derive(Deserialize, Serialize)]
pub struct AddressRange {
    pub start_address: Ipv4Addr,
    pub end_address: Ipv4Addr,
    pub subnet_mask: Ipv4Addr,
}

impl AddressRange {
    pub fn new(start_address: Ipv4Addr, end_address: Ipv4Addr, subnet_mask: Ipv4Addr) -> Self {
        AddressRange {
            start_address,
            end_address,
            subnet_mask,
        }
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            range: AddressRange {
                start_address: [192, 0, 0, 1].into(),
                end_address: [192, 0, 0, 100].into(),
                subnet_mask: [255, 255, 255, 0].into(),
            },
            configuration_path: String::new(),
        }
    }
}
impl Configuration {
    pub fn new(range: AddressRange, configuration_path: String) -> Configuration {
        Configuration {
            range,
            configuration_path,
        }
    }
    pub fn read(&self) {
        // todo read conf file
    }
}
