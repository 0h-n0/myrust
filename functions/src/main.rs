fn print_number(x: i32, y: i32){
    println!("sum is: {}", x + y);
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn _diverges() -> ! {
    panic!("This function never returns!");
}

fn main() {
    println!("Hello, world!");
    print_number(30, 6);
    let y = add_one(30);
    println!("{}", y);
    //let x: i32 = diverges();
    let f: fn(i32) -> i32 = add_one;
    println!("{}", f(1));
}
