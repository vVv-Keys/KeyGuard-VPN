// main.rs

use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write};
use rand::{thread_rng, Rng};
use openssl::symm::{Cipher, Crypter, Mode};

fn handle_client(mut stream: TcpStream, secret_key: &[u8]) {
    // Handle incoming client connection
    println!("Client connected!");

    // Set up encryption using AES-256
    let cipher = Cipher::aes_256_cbc();
    let mut encrypter = Crypter::new(cipher, Mode::Encrypt, secret_key, None).unwrap();
    let mut decrypter = Crypter::new(cipher, Mode::Decrypt, secret_key, None).unwrap();

    let mut buffer = [0; 1024];
    while let Ok(size) = stream.read(&mut buffer) {
        if size == 0 {
            println!("Client disconnected");
            return;
        }

        // Encrypt incoming data
        let mut encrypted_data = vec![0; size + cipher.block_size()];
        let mut pos = encrypter.update(&buffer[..size], &mut encrypted_data).unwrap();
        pos += encrypter.finalize(&mut encrypted_data[pos..]).unwrap();

        // Send encrypted data to client
        stream.write_all(&encrypted_data[..pos]).unwrap();

        // Decrypt incoming data
        let mut decrypted_data = vec![0; encrypted_data.len()];
        let mut pos = decrypter.update(&encrypted_data[..pos], &mut decrypted_data).unwrap();
        pos += decrypter.finalize(&mut decrypted_data[pos..]).unwrap();

        // Print decrypted data
        let decrypted_data = String::from_utf8_lossy(&decrypted_data[..pos]);
        println!("Received data from client: {}", decrypted_data);
    }
}

fn main() {
    // Generate a random secret key for encryption
    let mut rng = thread_rng();
    let mut secret_key = [0; 32];
    rng.fill(&mut secret_key);

    // Start listening for incoming connections
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");

    println!("Server listening on port 8080...");

    // Accept incoming connections and spawn a new thread to handle each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Spawn a new thread to handle the client connection
                let key = secret_key.clone();
                thread::spawn(move || {
                    handle_client(stream, &key);
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}
