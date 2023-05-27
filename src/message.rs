use std::fmt;
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

pub enum MessageType {
    // Client broadcast to locate available servers.
    DHCPDISCOVER,

    // Server to client in response to DHCPDISCOVER with
    // offer of configuration parameters
    DHCPOFFER,

    // Client message to servers either (a) requesting
    // offered parameters from one server and implicitly
    // declining offers from all others, (b) confirming
    // correctness of previously allocated address after,
    // e.g., system reboot, or (c) extending the lease on a
    // particular network address.
    DHCPREQUEST,

    // Server to client with configuration parameters,
    // including committed network address.
    DHCPACK,

    // Server to client indicating client's notion of network
    // address is incorrect (e.g., client has moved to new
    // subnet) or client's lease as expired
    DHCPNAK,

    // Client to server indicating network address is already
    // in use.
    DHCPDECLINE,

    // Client to server relinquishing network address and
    // cancelling remaining lease.
    DHCPRELEASE,

    // Client to server, asking only for local configuration
    // parameters; client already has externally configured
    // network address.
    DHCPINFORM,
}

#[derive(Debug, Clone)]
pub struct Message {
    // 1 = BOOTREQUEST, 2 = BOOTREPLY
    ///  Message op code / message type.
    op: u8,

    /// Hardware address type, see ARP section in "Assigned Numbers" RFC; e.g., '1' = 10mb ethernet.
    htype: u8,

    /// Hardware address length (e.g.  '6' for 10mb ethernet).
    hlen: u8,

    /// Client sets to zero, optionally used by relay agents when booting via a relay agent.
    hops: u8,

    /// Transaction ID, a random number chosen by the client, used by the client and server to associate messages and responses between a client and a server.
    xid: u32,

    /// Filled in by client, seconds elapsed since client began address acquisition or renewal process.
    secs: u16,

    /// Flags
    flags: u16,

    /// Client IP address; only filled in if client is in BOUND, RENEW or REBINDING state and can respond to ARP requests.
    ciaddr: u32,

    /// 'your' (client) IP address.
    yiaddr: u32,

    /// IP address of next server to use in bootstrap; returned in DHCPOFFER, DHCPACK by server.
    siaddr: u32,

    /// Relay agent IP address, used in booting via a relay agent.
    giaddr: u32,

    /// Client hardware address.
    chaddr: [u8; 16],

    /// Optional server host name, null terminated string.
    sname: [u8; 64],

    /// Boot file name, null terminated string; "generic" name or null in DHCPDISCOVER, fully qualified directory-path name in DHCPOFFER.
    file: [u8; 128],

    /// Optional parameters field.  See the options documents for a list of defined options.
    options: Vec<u8>,
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
        ciaddr: u32,
        yiaddr: u32,
        siaddr: u32,
        giaddr: u32,
        chaddr: [u8; 16],
        sname: [u8; 64],
        file: [u8; 128],
        options: Vec<u8>,
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
    pub fn serialize(&self) -> Vec<u8> {
        let xid_bytes = self.xid.to_be_bytes();
        let secs_bytes = self.secs.to_be_bytes();
        let flags_bytes = self.flags.to_be_bytes();
        let ciaddr_bytes = self.ciaddr.to_be_bytes();
        let yiaddr_bytes = self.yiaddr.to_be_bytes();
        let siaddr_bytes = self.siaddr.to_be_bytes();
        let giaddr_bytes = self.giaddr.to_be_bytes();
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
        res.extend(&self.options);
        res
    }
    pub fn deserialize(buffer: Vec<u8>) -> Message {
        println!("{:X?}", buffer);
        Message {
            op: buffer[0],
            htype: buffer[1],
            hlen: buffer[2],
            hops: buffer[3],
            xid: u32::from_be_bytes(
                buffer[4..8]
                    .try_into()
                    .expect("slice with incorrect length"),
            ),
            secs: u16::from_be_bytes(
                buffer[9..11]
                    .try_into()
                    .expect("slice with incorrect length"),
            ),
            flags: u16::from_be_bytes(
                buffer[12..14]
                    .try_into()
                    .expect("slice with incorrect length"),
            ),
            ciaddr: u32::from_be_bytes(
                buffer[15..19]
                    .try_into()
                    .expect("slice with incorrect length"),
            ),
            yiaddr: u32::from_be_bytes(
                buffer[20..24]
                    .try_into()
                    .expect("slice with incorrect length"),
            ),
            siaddr: u32::from_be_bytes(
                buffer[25..29]
                    .try_into()
                    .expect("slice with incorrect length"),
            ),
            giaddr: u32::from_be_bytes(
                buffer[30..34]
                    .try_into()
                    .expect("slice with incorrect length"),
            ),
            chaddr: buffer[35..51]
                .try_into()
                .expect("slice with incorrect length"),
            sname: buffer[52..116]
                .try_into()
                .expect("slice with incorrect length"),
            file: buffer[117..245]
                .try_into()
                .expect("slice with incorrect length"),
            options: buffer[246..buffer.len() - 1].to_vec(),
        }
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "op = {:X}\thtype = {:X}\thlen = {:X}\thops = {:X}\txid = {:X}\tsecs = {:X}\tflags = {:X}\tciaddr = {:X}\tyiaddr = {:X}\tsiaddr = {:X}\tgiaddr = {:X}\tchaddr = {:X?}\tsname = {:X?}\tfile = {:X?}\t options = {:X?}", self.op, self.htype, self.hlen, self.hops, self.xid, self.secs, self.flags, self.ciaddr, self.yiaddr, self.siaddr, self.giaddr, self.chaddr, self.sname, self.file, self.options)
    }
}
