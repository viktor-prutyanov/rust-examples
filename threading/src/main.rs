use std::thread;
use std::sync::{Arc, Mutex};

const N: usize = 10;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut threads = Vec::with_capacity(N);

    for _ in 0..N {
        let cnt = Arc::clone(&counter);
        threads.push(thread::spawn(move || {
            let mut c = cnt.lock().unwrap();

            *c += 1;
        }));
    }

    for (i, t) in threads.into_iter().enumerate() {
        match t.join() {
            Ok(_) => eprintln!("Thread #{} done!", i),
            Err(_) => panic!("Couldn't join thread #{}!", i)
        };
    }

    eprintln!("All done! Counter = {}", *counter.lock().unwrap());
}
