use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, UdpSocket};
use std::time::Duration; //use pretty_hex::pretty_hex;
use rand::Rng;

use crate::configuration::*;
use crate::message::*;

// Default lease set to 2h, maybe change that in configuration later ?
const DEFAULT_LEASE: Duration = Duration::new(7200, 0);


#[derive(PartialEq, Eq, Clone)]
pub struct Client {
    address: Ipv4Addr,
    hostname: String,
    lease: Duration,
}
impl Default for Client {
    fn default() -> Self {
        Client {
            address: Ipv4Addr::new(0, 0, 0, 0),
            hostname: String::new(),
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
        address: Ipv4Addr,
        hostname: String,
        lease: Duration,
    ) -> Self {
        Client {
            address,
            hostname,
            lease,
        }
    }
}

#[derive(Debug)]
pub enum ErrorPool {
    AddressAlreadyAllocated,
    AddressOutOfRange,
}

pub struct Pool {
    pub configuration: Configuration,
    pub reservation: HashMap<String, Client>,

}
impl Pool {
    pub fn new(configuration: Configuration) -> Self {
        Pool {
            configuration,
            reservation: HashMap::new(),
        }
    }
    fn reserve_ip(&mut self, mac: String) -> Result<&Client, ErrorPool> {
        let mut addr = (&self).random_addr();
        let mut is_free = false;
        while !is_free {
            addr = self.random_addr();
            is_free = match self.is_free(addr) {
                Ok(free) => {
                    // TODO : Try pinging before allocating
                    free
                },
                Err(e) => return Err(e),
            };
        }
        self.reservation.insert(mac.clone(), Client::init(addr));
        Ok(self.reservation.get(&mac).unwrap())

    }
    fn random_addr(&self) -> Ipv4Addr {
        let rnd_addr: u32 = rand::thread_rng().gen_range(self.configuration.range.start_address.into()..self.configuration.range.end_address.into());
        Ipv4Addr::from(rnd_addr)
    }
    fn is_free(&self, addr: Ipv4Addr) -> Result<bool, ErrorPool> {
        for (mac, client) in self.reservation.iter() {
            if client.address == addr {
                return Ok(false);
            }
        }

        Ok(true)
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
        )));

        loop {
            let mut buffer = [0; 576];
            let (num_byte, src_addr) = self
                .socket
                .recv_from(&mut buffer)
                .expect("ERR: An error occured while receiving bytes");

            //println!("Pretty hex : {}", pretty_hex(&buffer));
            let mut msg: Message = Message::deserialize(buffer.to_vec());

            println!("DEBUG: Message received : {}", &msg);

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
                    let mac: String = String::from_utf8_lossy(&msg.chaddr).to_string();
                    let client_offer: Client = match pool.reserve_ip(mac) {
                        Ok(client) => client.clone(),
                        Err(e) => {
                            println!("Error type : {:?}",e);
                            println!("Unable to reserve an IP : skipping.");
                            return;
                        },
                    };
                    let yiaddr = client_offer.address;
                    self.send_offer(&msg, src_addr, yiaddr);
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
    fn send_offer<T>(&self, source: &Message, dest: T, yiaddr: Ipv4Addr) -> Result<usize, std::io::Error>
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
        
        let siaddr: Ipv4Addr = Ipv4Addr::new(0,0,0,0);
        let mut response: Message = Message::new(
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
        // TODO : change response to init function
        // TODO : Add possibility to add option

        println!("DEBUG: message sended : {}\n", &response);
        // TODO : add apropriate option on the response message and use
        self.send(response, dest)
    }
}
