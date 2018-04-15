fn main() {
    // loop {
    //     // infinte loop
    //     println!("Hello, world!");
    // }
    let mut x = 5;
    let mut done = false;
    while !done {
        x += x - 3;
        println!("{}", x);
        if x % 5 == 0{
            done = true;
        }
    }
    for x in 0..10 {
        println!("{}", x);
    }
    let y = [0; 20];
    for i in y.iter() {
        println!("{}", i);
    }
    for (i,j) in (5..10).enumerate() {
        println!("i = {}, j = {}", i, j);
    }
    'outer: for x in 0..10 {
        'inner: for y in 0..10 {
            if x % 2 == 0 { continue 'outer; }
            if y % 2 == 0 { continue 'inner; }
            println!("x: {}, y: {}", x, y);
        }
    }
    for x in 0..10 {
        println!("{}", x);
    }
}
