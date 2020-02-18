#[derive(Debug, Clone)]
struct Foo(u32);

fn main() {
    let foo = Foo(2048);
    let bar = foo.clone();
    println!("Foo is {:?}", foo);
    println!("Bar is {:?}", bar);
}
