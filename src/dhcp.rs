use std::collections::HashMap;
use std::net::{Ipv4Addr, UdpSocket};
use std::time::Duration; //use pretty_hex::pretty_hex;

use crate::configuration::*;
use crate::message::*;

// Default lease set to 2h, maybe change that in configuration later ?
const DEFAULT_LEASE: Duration = Duration::new(7200, 0);

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum AddrState {
    FREE,
    OFFERED,
    BOUND,
}

#[derive(PartialEq, Eq, Clone)]
pub struct Client {
    state: AddrState,
    address: Ipv4Addr,
    hostname: String,
    mac: String,
    lease: Duration,
}
impl Default for Client {
    fn default() -> Self {
        Client {
            state: AddrState::FREE,
            address: Ipv4Addr::new(0, 0, 0, 0),
            hostname: String::new(),
            mac: String::new(),
            lease: DEFAULT_LEASE,
        }
    }
}
impl Client {
    pub fn init(address: Ipv4Addr) -> Self {
        Client {
            address,
            ..Default::default()
        }
    }
    pub fn new(
        state: AddrState,
        address: Ipv4Addr,
        hostname: String,
        mac: String,
        lease: Duration,
    ) -> Self {
        Client {
            state,
            address,
            hostname,
            mac,
            lease,
        }
    }
}

pub struct Pool {
    pub configuration: Configuration,
    pub reservation: Vec<Client>,
}
impl Pool {
    pub fn new(configuration: Configuration) -> Self {
        Pool {
            configuration,
            reservation: Vec::new(),
        }
    }
    pub fn init(mut self) -> Self {
        let start_addr: u32 = self.configuration.range.start_address.into();
        let end_addr: u32 = self.configuration.range.end_address.into();
        let size_addr: u32 = end_addr - start_addr;
        let mut reservation: Vec<Client> = Vec::new();
        for i in 0..size_addr {
            self.reservation.push(Client::init((start_addr + i).into()));
        }
        self
    }
    pub fn allocate(&mut self, addr: Ipv4Addr) -> Result<Ipv4Addr, &'static str> {
        todo!()
    }
    fn is_free(&self, addr: Ipv4Addr, hardware: String) -> bool {
        if self.configuration.range.end_address.cmp(&addr) != std::cmp::Ordering::Less
            && self.configuration.range.start_address.cmp(&addr) == std::cmp::Ordering::Less
        {
            return false;
        }

        for (index, client) in self.reservation.iter().enumerate() {
            if client.address == addr && client.state == AddrState::FREE {
                return true;
            }
        }
        false
    }
}

pub struct DhcpServer {
    socket: UdpSocket,
}
impl DhcpServer {
    pub fn new() -> DhcpServer {
        DhcpServer {
            socket: UdpSocket::bind("127.0.0.1:67").expect("ERR: unable to bind the port 67"),
        }
    }
    pub fn on_recv(&self) {
        let mut pool: Pool = Pool::new(Configuration::new(AddressRange::new(
            Ipv4Addr::new(192, 168, 0, 1),
            Ipv4Addr::new(192, 168, 0, 10),
            Ipv4Addr::new(255, 255, 255, 0),
        )))
        .init();

        loop {
            let mut buffer = [0; 576];
            let (num_byte, src_addr) = self
                .socket
                .recv_from(&mut buffer)
                .expect("ERR: An error occured while receiving bytes");

            //println!("Pretty hex : {}", pretty_hex(&buffer));
            let msg: Message = Message::deserialize(buffer.to_vec());

            println!("INFO: Message received : {}", &msg);

            let dhcp_type: MessageType = msg
                .options
                .options
                .iter()
                .find(|e| e.op_code == 53)
                .expect(
                    "DHCP Message MUST have a type field : msg did not contain 53 code in option field",
                    )
                .data[0]
                .into();

            match dhcp_type {
                MessageType::DHCPDISCOVER => {
                    // Server should respond with a DHCPOFFER message
                    self.send_offer(&msg, src_addr);
                },
                MessageType::DHCPREQUEST => {
                    // Server should respond with a DHCPACK message

                },
                MessageType::DHCPRELEASE => {
                    // Release address
                }
                _ => todo!(),
            }
        }
    }
    fn send<T>(&self, message: Message, dest: T) -> Result<usize, std::io::Error>
    where
        T: std::net::ToSocketAddrs,
    {
        self.socket.send_to(&message.serialize(), dest)
    }
    fn send_offer<T>(&self, source: &Message, dest: T) -> Result<usize, std::io::Error>
    where
        T: std::net::ToSocketAddrs,
    {
        /**
         * Field      DHCPOFFER
         * -----      ---------            
         * 'op'       BOOTREPLY           
         * 'htype'    (From "Assigned Numbers" RFC)
         * 'hlen'     (Hardware address length in octets)
         * 'hops'     0                    
         * 'xid'      'xid' from client DHCPDISCOVER message             
         * 'secs'     0                    
         * 'ciaddr'   0                    
         * 'yiaddr'   IP address offered to client            
         * 'siaddr'   IP address of next bootstrap server    
         * 'flags'    'flags' from client DHCPDISCOVER message              
         * 'giaddr'   'giaddr' from client DHCPDISCOVER message              
         * 'chaddr'   'chaddr' from client DHCPDISCOVER message             
         * 'sname'    Server host name or options           
         * 'file'     Client boot file name or options      
         * 'options'  options         
         */

        // Still to do but try if pooling works
        //let yiaddr: Ipv4Addr = todo!();
        //let siaddr: Ipv4Addr = todo!();
        
        let yiaddr: Ipv4Addr = Ipv4Addr::new(0,0,0,0);
        let siaddr: Ipv4Addr = Ipv4Addr::new(0,0,0,0);
        let response: Message = Message::new(
            OpCode::BOOTREPLY as u8,
            source.htype,
            source.hlen,
            0,
            source.xid,
            0,
            source.flags,
            Ipv4Addr::new(0, 0, 0, 0),
            yiaddr, // yiaddr
            siaddr, // siaddr
            source.giaddr,
            source.chaddr,
            [0u8; 64],
            [0u8; 128],
            OptionField::new(vec![]),
        );

        println!("INFO: message sended : {}\n", &response);
        // TODO : add apropriate option on the response message and use
        self.send(response, dest)
    }
}
