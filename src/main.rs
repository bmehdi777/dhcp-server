use std::net::UdpSocket;

mod message;

use message::{Message, MessageType};

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:67")?;

    loop {
        let mut buffer = [0; 4608];
        let (num_byte, src_addr) = socket.recv_from(&mut buffer)?;
        let buffer = &mut buffer[..num_byte];
        buffer.reverse();
        println!(
            "From : {} ; Received : {:X?}\nBuffer size : {}",
            src_addr, buffer, num_byte
        );
    }

    Ok(())
}
