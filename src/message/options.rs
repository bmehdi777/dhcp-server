use std::fmt;

#[derive(Debug, Clone)]
pub struct OptionField {
    pub magic_cookies: [u8; 4],
    pub options: Vec<OptionSubfield>,
}
#[derive(Debug, Clone)]
pub struct OptionSubfield {
    pub op_code: u8,
    pub op_len: u8,
    pub data: Vec<u8>,
}
#[repr(u8)]
#[derive(Debug, Clone)]
pub enum Option {
    Pad = 0,
    End = 255,
    SubnetMask = 1,
    TimeOffset = 2,
    Router = 3,
    TimeServer = 4,
    NameServer = 5,
    DomainNameServer = 6, // Domain Name System, RFC 1035
    LogServer = 7,
    CookieServer = 8,
    LPR = 9, // line printer servers (RFC 1179)
    ImpressServer = 10,
    ResourceLocationServer = 11,
    HostName = 12,
    BootFileSize = 13,
    MeritDumpFile = 14,
    DomainName = 15,
    SwapServer = 16,
    RootPath = 17,
    ExtensionsPath = 18,
    IPForwarding = 19,          // On/Off
    NonLocalSourceRouting = 20, // On/Off
    PolicyFilter = 21,
    MaximumDatagramReassemblySize = 22,
    DefaultIpTTL = 23,
    PathMTUAgingTimeout = 24,
    PathMTUPlateauTable = 25,
    InterfaceMTU = 26,
    AllSubnetsAreLocal = 27,
    BroadcastAddress = 28,
    PerformMaskDiscovery = 29,
    MaskSupplier = 30,
    PerformRouterDiscovery = 31,
    RouterSolicitationAddress = 32,
    StaticRoute = 33,
    TrailerEncapsulation = 34,
    ARPCacheTimeout = 35,
    EthernetEncapsulation = 36,
    TCPDefaultTTL = 37,
    TCPKeepaliveInterval = 38,
    TCPKeepaliveGarbage = 39,
    NetworkInformationServiceDomain = 40,
    NetworkInformationServers = 41,
    NetworkTimeProtocolServers = 42,
    VendorSpecificInformation = 43,
    NetBIOSOverTCPIPNameServer = 44,
    NetBIOSOverTCPIPDatagramDistributionServer = 45,
    NetBIOSOverTCPIPNodeType = 46,
    NetBIOSOverTCPIPScope = 47,
    XWindowSystemFontServer = 48,
    XWindowSystemDisplayManager = 49,
    RequestedIPAddress = 50,
    IPAddressLeaseTime = 51,
    OptionOverload = 52,
    DHCPMessageType = 53,
    ServerIdentifier = 54,
    ParameterRequestList = 55,
    Message = 56,
    MaximumDHCPMessageSize = 57,
    RenewalTimeValue = 58,
    RebindingTimeValue = 59,
    ClassIdentifier = 60,
    ClientIdentifier = 61,
    // I know there is other option but it will
    // be fine for the moment
}

impl OptionSubfield {
    pub fn new(op: Option, mut data: Vec<u8>) -> Result<OptionSubfield, String> {
        let op_len: u8;
        match op {
            Option::Pad | Option::End => {
                op_len = 0;
                data = Vec::new();
            }
            Option::SubnetMask
            | Option::TimeOffset
            | Option::SwapServer
            | Option::PathMTUAgingTimeout
            | Option::BroadcastAddress
            | Option::RouterSolicitationAddress
            | Option::ARPCacheTimeout
            | Option::TCPKeepaliveInterval
            | Option::NetBIOSOverTCPIPNameServer
            | Option::NetBIOSOverTCPIPDatagramDistributionServer
            | Option::RequestedIPAddress
            | Option::IPAddressLeaseTime
            | Option::ServerIdentifier
            | Option::RenewalTimeValue
            | Option::RebindingTimeValue => {
                op_len = 4;
                if data.len() != 4 {
                    return Err(format!("{:?} must have a data length of 4", op));
                }
            }
            Option::Router
            | Option::TimeServer
            | Option::NameServer
            | Option::DomainNameServer
            | Option::LogServer
            | Option::CookieServer
            | Option::LPR
            | Option::ImpressServer
            | Option::ResourceLocationServer
            | Option::NetworkInformationServers
            | Option::NetworkTimeProtocolServers
            | Option::XWindowSystemFontServer
            | Option::XWindowSystemDisplayManager => {
                op_len = u8::try_from(data.len()).expect("data is out of bound");
                if op_len / 4 != 0 && op_len < 4 {
                    return Err(format!(
                        "{:?} must have a data length of a multiple of 4",
                        op
                    ));
                }
            }
            Option::HostName
            | Option::MeritDumpFile
            | Option::DomainName
            | Option::RootPath
            | Option::ExtensionsPath
            | Option::DefaultIpTTL
            | Option::PerformMaskDiscovery
            | Option::MaskSupplier
            | Option::PerformRouterDiscovery
            | Option::NetworkInformationServiceDomain
            | Option::VendorSpecificInformation
            | Option::NetBIOSOverTCPIPScope
            | Option::ParameterRequestList
            | Option::Message
            | Option::ClassIdentifier => {
                op_len = u8::try_from(data.len()).expect("data is out of bound");
                if op_len < 1 {
                    return Err(format!("{:?} can't have empty data", op));
                }
            }
            Option::BootFileSize
            | Option::MaximumDatagramReassemblySize
            | Option::InterfaceMTU
            | Option::MaximumDHCPMessageSize => {
                op_len = 2;
                if data.len() != 2 {
                    return Err(format!("{:?} must have a data length of 2", op));
                }
            }
            Option::IPForwarding
            | Option::NonLocalSourceRouting
            | Option::AllSubnetsAreLocal
            | Option::TrailerEncapsulation
            | Option::EthernetEncapsulation
            | Option::TCPDefaultTTL
            | Option::TCPKeepaliveGarbage
            | Option::NetBIOSOverTCPIPNodeType
            | Option::OptionOverload
            | Option::DHCPMessageType => {
                op_len = 1;
                if data.len() != 1 {
                    return Err(format!("{:?} must have a data length of 1", op));
                }
            }
            Option::PolicyFilter | Option::StaticRoute => {
                op_len = u8::try_from(data.len()).expect("data is out of bound");
                if op_len / 8 != 0 && op_len < 8 {
                    return Err(format!(
                        "{:?} must have a data length of a multiple of 8",
                        op
                    ));
                }
            }
            Option::PathMTUPlateauTable => {
                op_len = u8::try_from(data.len()).expect("data is out of bound");
                if op_len / 2 != 0 && op_len < 2 {
                    return Err(format!(
                        "{:?} must have a data length of a multiple of 2",
                        op
                    ));
                }
            }
            _ => return Err("Unhandled case".into()),
        }

        Ok(OptionSubfield {
            op_code: op as u8,
            op_len,
            data,
        })
    }
    pub fn from_bytes(input: Vec<u8>) -> OptionSubfield {
        OptionSubfield {
            op_code: input[0],
            op_len: input[1],
            data: input[2..input.len()].to_vec(),
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![self.op_code, self.op_len];
        bytes.extend(&self.data);
        bytes
    }
}
impl OptionField {
    pub fn new(options: Vec<OptionSubfield>) -> OptionField {
        OptionField {
            magic_cookies: [99, 130, 83, 99], // set by rfc 1497
            options,
        }
    }
    pub fn from_bytes(input: Vec<u8>) -> OptionField {
        let magic_cookies: [u8; 4] = input[0..=3]
            .try_into()
            .expect("slice with incorrect length");
        let mut offset = 4;
        let mut options: Vec<OptionSubfield> = Vec::new();

        while offset < input.len() - 1 {
            match input[offset] {
                255 => {
                    break;
                }
                0 => {
                    // pad
                    offset += 1;
                }
                _ => {
                    let length: usize = input[offset + 1].into();
                    options.push(OptionSubfield::from_bytes(
                        input[offset..=offset + 1 + length].to_vec(),
                    ));
                    offset += length + 2;
                }
            }
        }

        OptionField {
            magic_cookies,
            options,
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = self.magic_cookies.to_vec();
        for op in self.options.iter() {
            bytes.extend(&op.to_bytes());
        }
        bytes.push(255); // termination bytes
        bytes
    }
}

impl fmt::Display for OptionSubfield {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "code: {} len: {} data: {:X?}",
            self.op_code, self.op_len, self.data
        )
    }
}

impl fmt::Display for OptionField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res: String = String::from(format!(
            "{{ magic_cookies : {:X?} subfield: [ ",
            self.magic_cookies
        ));
        for op in self.options.iter() {
            res = res + &format!("{{ {} }},", op);
        }
        write!(f, "{}] }}", res)
    }
}
