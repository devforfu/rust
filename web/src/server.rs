use std::{env, thread};
use std::net::TcpListener;
use std::thread::JoinHandle;

static HOST_ENV: &str = "RUST_HOST";
static PORT_ENV: &str = "RUST_PORT";

pub struct Config {
    host: String,
    port: String,
}

impl Config {
    pub fn new(host: &str, port: &str) -> Config {
        Config {
            host: String::from(host),
            port: String::from(port),
        }
    }

    pub fn from_env() -> Config {
        Config::new(
            &env::var(HOST_ENV).unwrap(),
            &env::var(PORT_ENV).unwrap(),
        )
    }

    pub fn default() -> Config {
        Config::new("127.0.0.1", "8080")
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

pub fn listen(config: &Config) -> JoinHandle<bool> {
    let address = config.address();

    let handle = thread::spawn(move || {
        let listener = TcpListener::bind(address).unwrap();
        let stream = listener.incoming().next().unwrap();
        stream.is_ok()
    });

    handle
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::net::TcpStream;

    #[test]
    fn test_server_config_new() {
        let config = Config::new("localhost", "8080");

        assert_eq!(config.address(), "localhost:8080");
    }

    #[test]
    fn test_server_config_from_env() {
        env::set_var(HOST_ENV, "address");
        env::set_var(PORT_ENV, "port");

        let config = Config::from_env();

        assert_eq!(config.address(), "address:port");
    }

    #[test]
    fn test_server_accepts_connections() {
        let config = Config::default();

        let handle = listen(&config);
        let _ = TcpStream::connect(config.address()).unwrap();
        let accepted = handle.join().is_ok();

        assert!(accepted);
    }
}