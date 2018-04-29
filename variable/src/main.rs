fn help(v1: &Vec<i32>) -> i32 {
    42
}

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
    let v1 = vec![1, 2, 3];
    help(&v1);
    println!("{}", v1[0]);
        
}
// borrowing rule
//
// 
