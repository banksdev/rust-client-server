mod client;
mod server;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).unwrap_or(&String::new()).as_ref() {
        "client" => match args.get(2) {
            Some(dest) => client::run(dest).unwrap(),
            None => println!("Please provide destination!"),
        },
        "server" => match args.get(2) {
            Some(addr) => server::run(addr).unwrap(),
            None => println!("Please provide server ip and port e.g. 127.0.0.1:80")
        },
        _ => println!("Usage: client | server [ip:port]")
    }
}
