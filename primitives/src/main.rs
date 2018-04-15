
fn main() {
    println!("Hello, world!");
    let x = true;
    let y: bool = false;
    if y {
        println!("Hello, world y 2!");        
    }else if x {
        println!("Hello, world x 2!");        
    }
    let c = 'あ';
    println!("c = {}", c);
    let _x8: i8 = 1;
    let _x16: i16 = 1;
    let _x32: i32 = 1;
    let _x64: i64 = 1;
    let _x8: u8 = 1;
    let _x16: u16 = 1;
    let _x32: u32 = 1;
    let _x64: u64 = 1;
    let _xisize: isize = 1;
    let _xusize: usize = 1;
    let _xf32: f32 = 1.0;
    let _xf64: f64 = 1.0;

    // Array
    let mut a = [1, 2, 3];
    a[0] = 10;
    println!("{}", a[0]);
    let a = [0, 1, 2, 3, 4, 5];
    let complete = &a[..];
    let middle = &a[1..4];
    println!("{}", complete[0]);
    println!("{}", middle[0]);    
    let s: &str = "hello";
    println!("{}", s);
    let t: (i32, &str) = (1, "world");
    let tuple = (1, 2, 3,);
    let xxx = tuple.0;
    let yyy = tuple.1;
    let zzz = tuple.2;
    println!("xxx is {}\nyyy is {}\nzzz is {}", xxx, yyy, zzz);
    
}
