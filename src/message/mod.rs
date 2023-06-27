pub mod options;

use crate::configuration::*;
use std::fmt;
use std::net::Ipv4Addr;

/**
* Format of a dhcp message
* 0                   1                   2                   3
  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
  |     op (1)    |   htype (1)   |   hlen (1)    |   hops (1)    |
  +---------------+---------------+---------------+---------------+
  |                            xid (4)                            |
  +-------------------------------+-------------------------------+
  |           secs (2)            |           flags (2)           |
  +-------------------------------+-------------------------------+
  |                          ciaddr  (4)                          |
  +---------------------------------------------------------------+
  |                          yiaddr  (4)                          |
  +---------------------------------------------------------------+
  |                          siaddr  (4)                          |
  +---------------------------------------------------------------+
  |                          giaddr  (4)                          |
  +---------------------------------------------------------------+
  |                                                               |
  |                          chaddr  (16)                         |
  |                                                               |
  |                                                               |
  +---------------------------------------------------------------+
  |                                                               |
  |                          sname   (64)                         |
  +---------------------------------------------------------------+
  |                                                               |
  |                          file    (128)                        |
  +---------------------------------------------------------------+
  |                                                               |
  |                          options (variable)                   |
  +---------------------------------------------------------------+
*/
#[derive(Debug, Clone)]
pub struct Message {
    // 1 = BOOTREQUEST, 2 = BOOTREPLY
    ///  Message op code / message type.
    pub op: u8,

    /// Hardware address type, see ARP section in "Assigned Numbers" RFC; e.g., '1' = 10mb ethernet.
    pub htype: u8,

    /// Hardware address length (e.g.  '6' for 10mb ethernet).
    pub hlen: u8,

    /// Client sets to zero, optionally used by relay agents when booting via a relay agent.
    pub hops: u8,

    /// Transaction ID, a random number chosen by the client, used by the client and server to associate messages and responses between a client and a server.
    pub xid: u32,

    /// Filled in by client, seconds elapsed since client began address acquisition or renewal process.
    pub secs: u16,

    /// Flags
    pub flags: u16,

    /// Client IP address; only filled in if client is in BOUND, RENEW or REBINDING state and can respond to ARP requests.
    pub ciaddr: Ipv4Addr,

    /// 'your' (client) IP address.
    pub yiaddr: Ipv4Addr,

    /// IP address of next server to use in bootstrap; returned in DHCPOFFER, DHCPACK by server.
    pub siaddr: Ipv4Addr,

    /// Relay agent IP address, used in booting via a relay agent.
    pub giaddr: Ipv4Addr,

    /// Client hardware address.
    pub chaddr: [u8; 16],

    /// Optional server host name, null terminated string.
    pub sname: [u8; 64],

    /// Boot file name, null terminated string; "generic" name or null in DHCPDISCOVER, fully qualified directory-path name in DHCPOFFER.
    pub file: [u8; 128],

    /// Optional parameters field.  See the options documents for a list of defined options.
    pub options: OptionField,
}

#[repr(u8)]
pub enum OpCode {
    BOOTREQUEST = 1,
    BOOTREPLY = 2,
}

#[derive(Debug, Clone)]
pub enum MessageType {
    // Client broadcast to locate available servers.
    DHCPDISCOVER = 1,

    // Server to client in response to DHCPDISCOVER with
    // offer of configuration parameters
    DHCPOFFER = 2,

    // Client message to servers either (a) requesting
    // offered parameters from one server and implicitly
    // declining offers from all others, (b) confirming
    // correctness of previously allocated address after,
    // e.g., system reboot, or (c) extending the lease on a
    // particular network address.
    DHCPREQUEST = 3,

    // Server to client with configuration parameters,
    // including committed network address.
    DHCPACK = 4,

    // Server to client indicating client's notion of network
    // address is incorrect (e.g., client has moved to new
    // subnet) or client's lease as expired
    DHCPNAK = 5,

    // Client to server indicating network address is already
    // in use.
    DHCPDECLINE = 6,

    // Client to server relinquishing network address and
    // cancelling remaining lease.
    DHCPRELEASE = 7,

    // Client to server, asking only for local configuration
    // parameters; client already has externally configured
    // network address.
    DHCPINFORM = 8,
}
impl From<u8> for MessageType {
    fn from(value: u8) -> Self {
        match value {
            1 => MessageType::DHCPDISCOVER,
            2 => MessageType::DHCPOFFER,
            3 => MessageType::DHCPREQUEST,
            4 => MessageType::DHCPACK,
            5 => MessageType::DHCPNAK,
            6 => MessageType::DHCPDECLINE,
            7 => MessageType::DHCPRELEASE,
            8 => MessageType::DHCPINFORM,
            _ => panic!("Couldn't parse {} as a DHCP Message", value),
        }
    }
}

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

impl OptionSubfield {
    pub fn new(op_code: u8, op_len: u8, data: Vec<u8>) -> OptionSubfield {
        OptionSubfield {
            op_code,
            op_len,
            data,
        }
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
    pub fn new(
        options: Vec<OptionSubfield>,
    ) -> OptionField {
        OptionField {
            magic_cookies: [99,130,83,99], // set by rfc 1497
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

impl Message {
    pub fn new(
        op: u8,
        htype: u8,
        hlen: u8,
        hops: u8,
        xid: u32,
        secs: u16,
        flags: u16,
        ciaddr: Ipv4Addr,
        yiaddr: Ipv4Addr,
        siaddr: Ipv4Addr,
        giaddr: Ipv4Addr,
        chaddr: [u8; 16],
        sname: [u8; 64],
        file: [u8; 128],
        options: OptionField,
    ) -> Message {
        Message {
            op,
            htype,
            hlen,
            hops,
            xid,
            secs,
            flags,
            ciaddr,
            yiaddr,
            siaddr,
            giaddr,
            chaddr,
            sname,
            file,
            options,
        }
    }
    pub fn on_message(&self) {
        let dhcp_type: MessageType = self
            .options
            .options
            .iter()
            .find(|e| e.op_code == 53)
            .expect(
                "DHCP Message MUST have a type field : message did not contain 53 code in option field",
            )
            .data[0]
            .into();

        match dhcp_type {
            MessageType::DHCPDISCOVER => {
                let conf = Configuration::default();
            }
            _ => todo!(),
        }
    }
    pub fn serialize(&self) -> Vec<u8> {
        let xid_bytes = self.xid.to_be_bytes();
        let secs_bytes = self.secs.to_be_bytes();
        let flags_bytes = self.flags.to_be_bytes();
        let ciaddr_bytes = self.ciaddr.octets().to_vec();
        let yiaddr_bytes = self.yiaddr.octets().to_vec();
        let siaddr_bytes = self.siaddr.octets().to_vec();
        let giaddr_bytes = self.giaddr.octets().to_vec();

        let mut res: Vec<u8> = vec![self.op, self.htype, self.hlen, self.hops];

        res.extend_from_slice(&xid_bytes);
        res.extend_from_slice(&secs_bytes);
        res.extend_from_slice(&flags_bytes);
        res.extend_from_slice(&ciaddr_bytes);
        res.extend_from_slice(&yiaddr_bytes);
        res.extend_from_slice(&siaddr_bytes);
        res.extend_from_slice(&giaddr_bytes);
        res.extend_from_slice(&self.chaddr);
        res.extend_from_slice(&self.sname);
        res.extend_from_slice(&self.file);
        res.extend(&self.options.to_bytes());

        res
    }
    pub fn deserialize(buffer: Vec<u8>) -> Message {
        let options_end = buffer[236..=buffer.len() - 1]
            .iter()
            .position(|&e| e == 255)
            .unwrap();
        Message {
            op: buffer[0],
            htype: buffer[1],
            hlen: buffer[2],
            hops: buffer[3],
            xid: u32::from_be_bytes(
                buffer[4..=7]
                    .try_into()
                    .expect("slice with incorrect length"),
            ),
            secs: u16::from_be_bytes(
                buffer[8..=9]
                    .try_into()
                    .expect("slice with incorrect length"),
            ),
            flags: u16::from_be_bytes(
                buffer[10..=11]
                    .try_into()
                    .expect("slice with incorrect length"),
            ),
            ciaddr: Ipv4Addr::new(buffer[12], buffer[13], buffer[14], buffer[15]),
            yiaddr: Ipv4Addr::new(buffer[16], buffer[17], buffer[18], buffer[19]),
            siaddr: Ipv4Addr::new(buffer[20], buffer[21], buffer[22], buffer[23]),
            giaddr: Ipv4Addr::new(buffer[24], buffer[25], buffer[26], buffer[27]),
            chaddr: buffer[28..=43]
                .try_into()
                .expect("slice with incorrect length"),
            sname: buffer[44..=107]
                .try_into()
                .expect("slice with incorrect length"),
            file: buffer[108..=235]
                .try_into()
                .expect("slice with incorrect length"),
            options: OptionField::from_bytes(buffer[236..=236 + options_end].to_vec()),
        }
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "op = {:X}\thtype = {:X}\thlen = {:X}\thops = {:X}\txid = {:X}\tsecs = {:X}\tflags = {:X}\tciaddr = {}\tyiaddr = {}\tsiaddr = {}\tgiaddr = {}\tchaddr = {:X?}\tsname = {:X?}\tfile = {:X?}\t options = {}\n", self.op, self.htype, self.hlen, self.hops, self.xid, self.secs, self.flags, self.ciaddr, self.yiaddr, self.siaddr, self.giaddr, self.chaddr, self.sname, self.file, self.options)
    }
}
