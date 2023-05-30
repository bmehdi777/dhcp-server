use std::collections::HashMap;
use std::net::{Ipv4Addr, UdpSocket};

use crate::configuration::*;
use crate::message::*;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum PoolAddrState {
    FREE,
    BOUND
}

pub struct Pool {
    item: HashMap<Client, PoolAddrState>, // name to changes, just trying things here
}

#[derive(PartialEq, Eq, Hash)]
pub struct Client {
    address: Ipv4Addr,
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
        loop {
            let mut buffer = [0; 576];
            let (num_byte, src_addr) = self
                .socket
                .recv_from(&mut buffer)
                .expect("ERR: An error occured while receiving bytes");
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
                    let mut pool_available: HashMap<Client, PoolAddrState> = HashMap::new();
                    pool_available.insert(
                        Client {
                            address: Ipv4Addr::new(127, 0, 0, 2),
                        },
                        PoolAddrState::FREE,
                    );
                    let t: Vec<&Client> = pool_available.iter().filter_map(|(key, &value)| if value == PoolAddrState::FREE { Some(key)} else {None}).collect();
                }
                _ => todo!(),
            }
            println!("Message debug : {}", msg);
        }
    }
}
