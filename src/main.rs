use std::net::UdpSocket;

enum MessageType {
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

struct Message {
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
    sname: [char; 64],

    /// Boot file name, null terminated string; "generic" name or null in DHCPDISCOVER, fully qualified directory-path name in DHCPOFFER.
    file: [char; 128],

    /// Optional parameters field.  See the options documents for a list of defined options.
    options: Vec<u8>,

}

fn main() -> std::io::Result<()>{
    let socket = UdpSocket::bind("127.0.0.1:67")?;

    loop {
        let mut buffer = [0;4608];
        let (num_byte, src_addr) = socket.recv_from(&mut buffer)?;
        let buffer = &mut buffer[..num_byte];
        buffer.reverse();
        println!("From : {} ; Received : {:X?}\nBuffer size : {}", src_addr, buffer, num_byte);
    }

    Ok(())
}
