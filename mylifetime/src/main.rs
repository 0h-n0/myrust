#[derive(Debug)]
struct Human<'a> {
    height: f64,
    weight: &'a f64,
    name: String
}
fn create_f64() -> f64 {
    1.
}

fn create_human(weight: &f64) -> Human {
    Human {
        height: 175.0,
        weight: weight,
        name: "taro".to_string()
    }
}

fn main() {
    {
        let x = create_f64();
        {
            let mut a = Human {
                height: 175.0,
                weight: &x,
                name: "taro".to_string()
            };
            println!("Hello, world!, {:?}", &a);
            a.height = 3.;
            println!("Hello, world!, {:?}", &a);
        }
        println!("Hello, world!, {}", &x);
    }

}
