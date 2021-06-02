use std::{fs, env, thread};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use web::{Config, ThreadPool};

fn main() {
    let config = Config::default();
    let listener = TcpListener::bind(config.address()).unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn read_template(name: &str) -> String {
    let root = String::from(env::current_dir().unwrap().to_str().unwrap());
    let path = format!("{}/templates/{}.html", root, name);
    fs::read_to_string(path).unwrap()
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep= b"GET /sleep HTTP/1.1\r\n";

    let (status, template) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404")
    };

    let contents = read_template(template);

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
