use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Mutex is an abbreviation for mutual exclusion, as in,
    // a mutex allows only one thread to access some data at
    // any given time. This is useful if we want multiple threads
    // to share memory.

    let m = Mutex::new(5);

    {
        // We must use lock in order to access the value
        // that Mutex is wrapping.
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
