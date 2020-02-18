use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let c = Arc::new(Mutex::new(0));
    let mut hs = vec![];

    for i in 0..100 {
        let c = Arc::clone(&c);
        let h = thread::spawn(move || {
            println!("{}: thread", i);
            let mut num = c.lock().unwrap();
            *num += i;
        });
        hs.push(h);
    }
    for h in hs {
        h.join().unwrap();
    }
    println!("Result: {}", *c.lock().unwrap());
}
