use itertools::{interleave, Itertools};

fn main() {
    let it = (1..3).interleave(vec![-1, -2]);
    for i in it {
        println!("{:?}", i);
    }
    for ele in interleave(&[1, 2, 3], &[2, 3, 4]) {
        println!("{:?}", ele);
    }
}
