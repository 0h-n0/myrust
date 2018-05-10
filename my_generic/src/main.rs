
fn gratest_number(a: &[i32]) -> i32 {
    let mut largest: i32 = a[0];

    for (idx, &i) in a.iter().enumerate() {
        if i > largest {
            largest = i;
        }
    }
    largest
}

fn main() {
    let a = vec![1, 2, 3, 4, 5, 40];
    println!("{}", gratest_number(&a));
}
