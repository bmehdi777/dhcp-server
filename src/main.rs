#![allow(warnings)] // warning annoying
use std::net::UdpSocket;

mod message;
mod configuration;

use message::{Message, MessageType};

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:67")?;
    // May be usefull when I will look at the broadcast flag
    //socket.set_broadcast(true)?; 
    println!("Listening on 127.0.0.1:67");
    println!("Default configuration example \n{}", configuration::Configuration::default().to_toml());
    loop {
        let mut buffer = [0; 576];
        let (num_byte, src_addr) = socket.recv_from(&mut buffer)?;
        let msg: Message = Message::deserialize(buffer.to_vec());
        println!("Message debug : {}", msg);
    }

    //Ok(())
}
