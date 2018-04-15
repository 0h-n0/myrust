fn main() {
    let (x, y) = (1, 2);
    println!("{}, {}", x, y);
    let z: i32 = 5;
    println!("{}", z);
    let mut xx = 20;
    println!("{}", xx);    
    xx = 30;
    println!("{}", xx);
    let xx = 50;
    // shadowing
    println!("{}", xx);

    let mut x: i32 = 1;
    x = 7;
    x = 5;
    let x = x;
    println!("{}", x);
}
