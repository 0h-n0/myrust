use std::sync::mpsc;
use std::thread;

fn main() {
    let mut c = vec![];
    let mut recivers = vec![];

    for i in 0..10 {
        let (tx, rx) = mpsc::channel();
        c.push(thread::spawn(move || {
            tx.send(42).unwrap();
        }));
        recivers.push(rx);
    }
    for r in recivers {
        println!("go {}", r.recv().unwrap());
    }
}
