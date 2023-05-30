#![allow(warnings)] // warning annoying

mod configuration;
mod dhcp;
mod message;

fn main() {
    //println!("Default configuration example \n{}", configuration::Configuration::default().to_toml());
    // May be usefull when I will look at the broadcast flag
    //socket.set_broadcast(true)?;
    let server = dhcp::DhcpServer::new();
    server.on_recv();
}
