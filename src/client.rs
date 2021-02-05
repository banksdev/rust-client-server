use std::io::prelude::*;
use std::net::TcpStream;

pub fn run(dest: &String) -> std::io::Result<()> {
    let mut stream = TcpStream::connect(dest)?;

    const DATA:&[u8] = b"Hello, World";

    println!("Sending: {:?}...", DATA);
    stream.write(DATA)?;

    Ok(())
}