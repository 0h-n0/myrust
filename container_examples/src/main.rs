use std::collections::HashMap;

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
                    

fn main() {
    let v1: Vec<i32> = Vec::new();
    {
        let mut v2 = vec![1, 2, 3];
        v2.push(5);
        v2.push(6);
        v2.push(7);
        v2.push(8);
    }
    let v1 = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v1[2];
    let third: Option<&i32> = v1.get(2);
    //println!("{}", v2[0]);
    println!("{:?}", third);
    println!("Hello, world!");
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    let mut s = String::new();
    let data = "initial contents";
    let s = data.to_string();
    println!("{}", s);
    let ss = String::from("second contents");
    println!("{}", ss);

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("s3 is {}", s3);
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s);
    let s1 = String::from("tic");    
    let s4 = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s4);
    println!("{}", s1);    
    let len = s1.len();
    let hello = "アリガトウ";
    let h = &hello[0..9];
    
    println!("{}", len);
    println!("{}", h);
    for c in "abcdefg".chars() {
        println!("{}", c);
    }
    for c in hello.bytes() {
        println!("{}", c);
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 10);
    println!("{:?}", scores);
    let terms = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = terms.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", scores);

    let filed_name = String::from("Favorite color");
    let filed_value = String::from("Blue");

    let mut map = HashMap::new();

    map.insert(&filed_name, filed_value);

    // println!("{}", filed_name);
    println!("{:?}", map);
    println!("{:?}", map.get(&filed_name));
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    let mut scores2 = HashMap::new();
    scores2.insert(String::from("Blue"), 10);
    scores2.entry(String::from("Yellow")).or_insert(50);
    scores2.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores2);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
        
}
