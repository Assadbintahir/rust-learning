use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let my_vec = Arc::new(Mutex::new(vec![]));

    let handles = (0..10).map(|number| {
        let cloned_vec = my_vec.clone();
        thread::spawn(move || {
            let mut vector = cloned_vec.lock().unwrap();
            vector.push(number);
        })
    });

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{:?}", my_vec.lock().unwrap());
}