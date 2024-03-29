// client.rs
use std::error::Error;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tun_tap::{Iface, Mode};

async fn handle_server(mut stream: TcpStream, mut tun: Iface) -> Result<(), Box<dyn Error>> {
    let mut buf = [0u8; 1504]; // Maximum size of Ethernet frame

    loop {
        let nbytes = stream.read(&mut buf).await?;
        tun.send(&buf[..nbytes])?;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create a TUN interface
    let mut tun = Iface::new("tun0", Mode::Tun)?;

    // Connect to the VPN server
    let stream = TcpStream::connect("127.0.0.1:8080").await?;
    println!("Connected to server");

    // Handle communication with server
    handle_server(stream, tun).await?;

    Ok(())
}
