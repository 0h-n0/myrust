enum IpAddrKind {
    V4(String),
    V6(String),
}

enum Option<T> {
    Some(T),
    None,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 1,
        Coin::Dime => 1,
        Coin::Quarter => 1,        
    }
}

fn main() {
    println!("Hello, world!");
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    
}
