use std::net::{TcpListener, TcpStream};
use std::io::Read;

fn handle_client(stream: TcpStream) {
    println!("Handling stream... {}", stream.peer_addr().unwrap());

    let mut stream = stream.try_clone().unwrap();
    let mut buf = String::new();

    match stream.read_to_string(&mut buf) {
        Ok(_) => println!("Received: {:?}", buf),
        Err(e) => println!("Error: {}", e)
    }
}

pub fn run(addr: &String) -> std::io::Result<()> {
    println!("Staring server at {}", addr);
    let listener = TcpListener::bind(addr)?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }

    Ok(())
}