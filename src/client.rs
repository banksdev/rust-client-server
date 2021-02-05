use std::io::prelude::*;
use std::net::TcpStream;

pub fn run(dest: &String) -> std::io::Result<()> {
    let mut stream = TcpStream::connect(dest)?;
    println!("Starting client...\n  from: {}\n  to: {}", 
        stream.local_addr().unwrap(), 
        stream.peer_addr().unwrap());

    const DATA:&[u8] = b"Hello, World";

    println!("Sending: {:?}...", DATA);
    stream.write(DATA)?;

    Ok(())
}