pub const PROVER_ADDRESS:&str = "127.0.0.1:8080";
pub const CLIENT_ADDRESS:&str = "127.0.0.1:8181";

mod client;
mod server;


fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).unwrap_or(&String::new()).as_ref() {
        "client" => client::run().unwrap(),
        "server" => server::run().unwrap(),
        _ => println!("Usage: client | server [ip:port]")
    }
}
