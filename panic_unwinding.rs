use std::thread;

fn alice() -> thread::JoinHandle<()> {
    thread::spawn(move || {
        bob();
    })
}

fn bob() {
    malice();
}

fn malice() {
    panic!("malice is panicking!");
}

fn main() {
    let child = alice();
    let o = child.join();
    println!("{:?}", o);
    //bob();
    println!("This is unreachable code");
}
