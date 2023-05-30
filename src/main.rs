#![allow(warnings)] // warning annoying

mod message;
mod configuration;
mod dhcp;

fn main() {
    let server = dhcp::DhcpServer::new();
    server.on_recv();
}
