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
}
