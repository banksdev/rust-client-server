use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

pub fn run() -> std::io::Result<()> {
    println!("Trying to connect to {}...", crate::PROVER_ADDRESS);
    let mut stream = TcpStream::connect(crate::PROVER_ADDRESS)?;
    println!("Starting client...\n  from: {}\n  to: {}", 
        stream.local_addr().unwrap(), 
        stream.peer_addr().unwrap());

    const ATTEST_FLAG:&str = "attest";
    
    let data = format!("{};{}", &ATTEST_FLAG, crate::CLIENT_ADDRESS);

    println!("Sending: {:?} to {:?}...", data, crate::PROVER_ADDRESS);
    stream.write(data.as_bytes())?;
    stream.shutdown(std::net::Shutdown::Both)?;

    let listener = TcpListener::bind(crate::CLIENT_ADDRESS)?;
    
    println!("Awiting reply on {:?} from {:?}...", crate::CLIENT_ADDRESS, crate::PROVER_ADDRESS);
    // handle response
    for stream in listener.incoming() {
        handle_response(stream?);
        break;
    }

    Ok(())
}

fn handle_response(stream:TcpStream) {
    println!("Handling stream from: {}", stream.peer_addr().unwrap());
    let mut stream = stream.try_clone().unwrap();
    let mut challenge = String::new();

    match stream.read_to_string(&mut challenge) {
        Ok(_) => println!("Received: {:?}", challenge),
        Err(e) => println!("Error: {}", e)
    }
}