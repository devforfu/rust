use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;
use rand::Rng;
use std::time::Duration;
use std::thread::JoinHandle;

fn strings_generator(total: usize, sender: Sender<String>) -> JoinHandle<()> {
    let durations = random_durations(total);

    let handle = thread::spawn(move || {
        for (i, x) in durations.iter().enumerate() {
            thread::sleep(Duration::from_millis(*x));
            let generated = format!("x{}", i+1);
            sender.send(generated).unwrap();
        }
    });

    handle
}

fn numbers_generator(start: usize, end: usize, sender: Sender<usize>) -> JoinHandle<()> {
    let durations = random_durations(end - start);

    let items: Vec<usize> = (start..end).into_iter().collect();

    let handle = thread::spawn(move || {
        for (i, x) in durations.iter().enumerate() {
            thread::sleep(Duration::from_millis(*x));
            sender.send(items[i]).unwrap();
        }
    });

    handle
}

fn random_durations(total: usize) -> Vec<u64> {
    let mut rng = rand::thread_rng();
    (0..total).map(|_| rng.gen_range(1, 10)).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_communication_via_channel() {
        let (tx, rx) = mpsc::channel();

        let handle = strings_generator(5, tx);
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

    #[test]
    fn test_communication_via_channel_better() {
        let (tx, rx) = mpsc::channel();

        let handle = strings_generator(5, tx);
        let collected: Vec<String> = rx.iter().collect();
        handle.join().unwrap();

        assert_eq!(collected, vec!["x1", "x2", "x3", "x4", "x5"]);
    }

    #[test]
    fn test_receiving_values_from_many_producers() {
        let (tx1, rx) = mpsc::channel();
        let tx2 = tx1.clone();

        let h1 = numbers_generator(1, 5, tx1);
        let h2 = numbers_generator(5, 10, tx2);
        let mut collected: Vec<usize> = rx.iter().collect();
        collected.sort();
        h1.join().unwrap();
        h2.join().unwrap();

        assert_eq!(collected, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}