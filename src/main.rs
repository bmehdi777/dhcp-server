#![allow(warnings)] // warning annoying

mod configuration;
mod server;
mod message;

fn main() {
    //println!("Default configuration example \n{}", configuration::Configuration::default().to_toml());
    // May be usefull when I will look at the broadcast flag
    //socket.set_broadcast(true)?;
    let server = server::DhcpServer::new();
    println!("INFO: server started on 127.0.0.1:67\n");
    server.on_recv();
}
