#[derive(Debug)]
enum Tensor<T: std::fmt::Debug> {
    Tensor1(Vec<T>),
    Tensor2(Vec<Vec<T>>),
    Tensor3(Vec<Vec<Vec<T>>>),
    name(String),
}

fn print_tensor<T: std::fmt::Debug>(t: &Tensor<T>) {
    println!("{:?}", t);
}

fn main() {
    let t = Tensor::Tensor1(vec![1, 2, 3]);
    print_tensor(&t);

    let t2 = Tensor::Tensor2(vec![vec![1, 2, 3], vec![1, 2, 3]]);
    print_tensor(&t2);
    let t3 = Tensor::Tensor3(vec![
        vec![vec![1, 2, 3], vec![1, 2, 3]],
        vec![vec![1, 2, 3], vec![1, 2, 3]],
    ]);
    print_tensor(&t3);
}
