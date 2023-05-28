#![allow(warnings)] // warning annoying
use std::net::UdpSocket;

mod message;

use message::{Message, MessageType};

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:67")?;
    println!("Listening on 127.0.0.1:67");
    loop {
        let mut buffer = [0; 576];
        let (num_byte, src_addr) = socket.recv_from(&mut buffer)?;
        let msg: Message = Message::deserialize(buffer.to_vec());
        println!("Message debug : {}", msg);
    }

    //Ok(())
}
