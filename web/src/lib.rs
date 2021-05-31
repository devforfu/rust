use std::{env, thread};
use std::net::TcpListener;
use std::thread::JoinHandle;
use std::sync::{mpsc, Arc, Mutex};

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

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

pub struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {} got a job; executing.", id);

            job();
        });

        Worker { id, thread }
    }
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