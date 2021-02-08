use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;

fn handle_client(stream: TcpStream) {
    println!("Handling stream from: {}", stream.peer_addr().unwrap());

    let mut stream = stream.try_clone().unwrap();
    let mut challenge = String::new();

    match stream.read_to_string(&mut challenge) {
        Ok(_) => println!("Received: {:?}", challenge),
        Err(e) => println!("Error: {}", e)
    }


    let mut challenge = challenge.split(";");
    let _ = challenge.next().unwrap();
    let client = challenge.next().unwrap();

    println!("Trying to connect to client at {}", client);
    if let Ok(mut client_stream) = TcpStream::connect(client) {
        println!("Sending attestation reply...");
        client_stream.write(b"Attestation Succesful").unwrap();
        client_stream.shutdown(std::net::Shutdown::Both).unwrap();
    }
    else {
        println!("Failed to connect...");
    }
}

pub fn run() -> std::io::Result<()> {
    println!("Staring server at {}", crate::PROVER_ADDRESS);
    let listener = TcpListener::bind(crate::PROVER_ADDRESS)?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }

    Ok(())
}