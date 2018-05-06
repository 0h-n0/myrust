use std::collections::HashMap;
    
fn sum(s: &Vec<i32>) -> i32 {
    let mut s0 = 0;
    for v in s.iter() {
        s0 += v;
    }
    s0
}
fn mean(s: &Vec<i32>) -> f32 {
    (s.len() as f32 / sum(&s) as f32)
}
fn median(s: &Vec<i32>) -> i32 {
    let mut s2 = s.clone();
    s2.sort();
    let idx: i32 = s2.len() as i32 / 2 as i32;
    idx
}
fn mode(s: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    for i in s.iter() {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }
    let mut maxvalue: i32 = 0;
    let mut maxkey: i32 = 0;    
    for (k, i) in &map {
        if *i > maxvalue {
            maxvalue = *i;
            maxkey = **k;
        }
    }
    maxkey
}

fn main() {
    let s: Vec<i32> = [1, 5, 2, 3, 4, 4, 4].to_vec();
    println!("{}", sum(&s));
    println!("{}", mean(&s));
    println!("{}", median(&s));
    println!("{}", mode(&s));
}
