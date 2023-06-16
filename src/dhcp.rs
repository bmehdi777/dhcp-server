use std::collections::HashMap;
use std::net::{Ipv4Addr, UdpSocket};
//use pretty_hex::pretty_hex;

use crate::configuration::*;
use crate::message::*;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum PoolAddrState {
    FREE,
    BOUND,
}

pub struct Pool {
    pub configuration: Configuration,
    pub addr_used: HashMap<Ipv4Addr, String>,
}
impl Pool {
    pub fn new(configuration: Configuration) -> Pool {
        Pool {
            configuration, addr_used: HashMap::new(), }
    }
    pub fn use_addr(&mut self, addr: Ipv4Addr, hardware_addr: String) -> Result<(), &'static str> {
        if let Some(_) = self.addr_used.get(&addr) {
            return Err("ERR: the address is already used");
        }
        self.addr_used.insert(addr, hardware_addr);
        Ok(())
    }
    pub fn release_addr(&mut self, addr: Ipv4Addr) -> Result<(), &'static str> {
        if let None = self.addr_used.remove(&addr) {
            return Err("ERR: unable to find the address to remove");
        }
        Ok(())
    }
    pub fn choose_addr(&mut self) -> Result<Ipv4Addr, &'static str> {
        unimplemented!();
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
            Ipv4Addr::new(192, 168, 0, 1).octets(),
            Ipv4Addr::new(192, 168, 0, 10).octets(),
            [255, 255, 255, 0],
        )));

        loop {
            let mut buffer = [0; 576];
            let (num_byte, src_addr) = self
                .socket
                .recv_from(&mut buffer)
                .expect("ERR: An error occured while receiving bytes");

            //println!("Pretty hex : {}", pretty_hex(&buffer));
            let msg: Message = Message::deserialize(buffer.to_vec());

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


                }
                _ => todo!(),
            }
            println!("Message debug : {}", msg);
        }
    }
    fn send<T>(&self, message: Message, dest: T)
    where
        T: std::net::ToSocketAddrs,
    {
        self.socket.send_to(&message.serialize(), dest);
    }
    fn send_offer<T>(&self, source: Message, dest: T)
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

        let response: Message = Message::new(OpCode::BOOTREPLY as u8, source.htype, source.hlen, 0, source.xid, 0, source.flags, Ipv4Addr::new(0,0,0,0), todo!(), todo!(), source.giaddr, source.chaddr, [0u8; 64], [0u8; 128], OptionField::new(vec![]));

        // TODO : add apropriate option on the response message and use
        // self.send to send it over the network
    }
}
