use std::net::TcpListener;
mod server;

fn main() {
    let config = server::Config::default();

    let listener = TcpListener::bind(config.address()).unwrap();

    for stream in listener.incoming() {
        let _ = stream.unwrap();

        println!("Connection established!");
    }
}
