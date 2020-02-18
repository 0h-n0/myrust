use std::thread;

fn main() {
    let mut c = vec![];
    for i in 0..10 {
        c.push(thread::spawn(move || {
            println!("thread number {}", i);
        }));
    }
    println!("{:?}", c);
    for j in c {
        j.join();
    }
    for j in 0..10 {
        println!("main thread");
    }
}
