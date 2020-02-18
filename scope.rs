#[derive(Debug, Clone, Copy)]
struct Hello;

fn main() {
    let a = Hello {};
    {
        let b = a;
        println!("{:?}", b);
    }
    println!("{:?}", a);
}
