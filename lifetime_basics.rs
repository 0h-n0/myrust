#[derive(Debug)]
struct Hello(u32);

struct SomeRef<'a, T> {
    part: &'a T,
}

fn main() {
    let b = Hello(43);
    let a = SomeRef { part: &b };
    println!("{:?}", a.part);
    let c = b;
    println!("{:?}", c);
}
