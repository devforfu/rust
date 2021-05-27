mod channels;

use std::thread;
use std::time::Duration;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    let durations: Vec<u64> = (1..10).map(|x| rng.gen_range(1, x+1)).collect();

    let handle = thread::spawn(move || {
        for j in 1..10 {
            let duration = durations[j-1];
            println!("hi number {} from the spawned thread! (sleep: {})", j, duration);
            thread::sleep(Duration::from_millis(duration));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(5));
    }

    handle.join().unwrap();
}
