fn main() {
    let x = 5;
    
    if x == 5 {
        println!("Hello, world!");
    } else if x == 6 {
        println!("6 Hello, world!");        
    } else {
        println!("Not Hello, world!");
    }
    let y = if x == 5 {
        10
    } else {
        15
    };
    println!("y = {}", y);    
}
