use std::net::UdpSocket;

fn main() -> std::io::Result<()>{
    let socket = UdpSocket::bind("127.0.0.1:67")?;

    loop {
        let mut buffer = [0;2496];
        let (num_byte, src_addr) = socket.recv_from(&mut buffer)?;
        let buffer = &mut buffer[..num_byte];
        buffer.reverse();
        println!("From : {} ; Received : {:X?}", src_addr, buffer);
    }

    Ok(())
}
