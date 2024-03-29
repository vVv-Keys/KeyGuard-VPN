// server.rs
use std::error::Error;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tun_tap::{Iface, Mode};

async fn handle_client(mut stream: TcpStream, mut tun: Iface) -> Result<(), Box<dyn Error>> {
    let mut buf = [0u8; 1504]; // Maximum size of Ethernet frame

    loop {
        let nbytes = tun.recv(&mut buf)?;
        stream.write_all(&buf[..nbytes]).await?;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create a TUN interface
    let mut tun = Iface::new("tun0", Mode::Tun)?;

    // Create a TCP listener
    let listener = TcpListener::bind("0.0.0.0:8080").await?;

    println!("Server listening on port 8080");

    loop {
        let (stream, _) = listener.accept().await?;
        println!("New client connected!");

        let tun_clone = tun.clone();
        tokio::spawn(async move {
            if let Err(err) = handle_client(stream, tun_clone).await {
                eprintln!("Error handling client: {}", err);
            }
        });
    }
}
