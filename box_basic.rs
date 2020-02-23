fn box_ref<T>(b: T) -> Box<T> {
    let a = b;
    Box::new(a)
}

#[derive(Debug)]
struct Foo;

fn main() {
    let boxed_one = Box::new(Foo);
    let unboxed_one = *boxed_one;
    let a = box_ref(unboxed_one);
    println!("{:?}", a);
}
