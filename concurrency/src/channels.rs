use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;
use rand::Rng;
use std::time::Duration;
use std::thread::JoinHandle;

fn numbers_generator(total: usize, sender: Sender<String>) -> JoinHandle<()> {
    let mut rng = rand::thread_rng();

    let durations: Vec<u64> = (0..total).map(|_| rng.gen_range(1, 10)).collect();

    let handle = thread::spawn(move || {
        for (i, x) in durations.iter().enumerate() {
            thread::sleep(Duration::from_millis(*x));
            let generated = format!("x{}", i+1);
            sender.send(generated).unwrap();
        }
    });

    handle
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_communication_via_channel() {
        let (tx, rx) = mpsc::channel();

        let handle = numbers_generator(5, tx);
        let mut collected: Vec<String> = Vec::new();
        loop {
            match rx.recv() {
                Ok(v) => collected.push(v),
                Err(_) => {
                    handle.join().unwrap();
                    break
                },
            }
        }

        assert_eq!(collected, vec!["x1", "x2", "x3", "x4", "x5"]);
    }
}