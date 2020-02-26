use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);

    let _incrementaed_value = match map.get("one") {
        Some(val) => val + 1,
        None => 0,
    };
    let incrementaed_value = if let Some(v) = map.get("one") {
        v + 1
    } else {
        0
    };
    println!("{}", incrementaed_value);
}
