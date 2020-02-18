#[derive(Debug)]
enum Food {
    Cake,
    Pizza,
    Salad,
}

#[derive(Debug)]
struct Bag {
    food: Food,
}

fn main() {
    let bag = Bag { food: Food::Pizza };
    match bag.food {
        Food::Cake => println!("I go cake"),
        ref a => println!("I got {:?}", a),
    }
    println!("{:?}", bag);
}
