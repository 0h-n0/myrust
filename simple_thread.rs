use std::thread;

fn main() {
    thread::spawn(|| println!("GREAETING, HUMANS"))
        .join()
        .unwrap();
}
