use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

fn show(table: &Table) {
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!("  {}", work);
        }
    }
}

static mut STASH: &i32 = &128;

fn main() {
    let mut table = Table::new();

    table.insert("Gesualdo".to_string(),
                 vec!["many madrigals".to_string(),
                      "Tenebrae REsponsoria".to_string()]);

    show(&table);
    
    println!("Hello, world!");
    let mut x = 32;
    let m = &mut x;
    *m += 32;
    assert!(*m == 64);

    struct Anime { name: &'static str, bechdel_pass: bool}
    let aria = Anime { name: "Aria: The Animation", bechdel_pass: true};
    let anime_ref = &aria;
    assert_eq!(anime_ref.name, "Aria: The Animation");

    let mut v = vec![1973, 1968];
    v.sort();
    (&mut v).sort();

    #[derive(Debug)]
    struct Point {x: i32, y: i32 }

    let point = Point { x: 1000, y: 729 };
    let r: &Point = &point;
    let rr: &&Point = &&point;
    let rrr: &&&Point = &&&point;    
    println!("{:?}", *r);
    println!("{:?}", **rr);
    println!("{:?}", ***rrr);

    fn factorial(n: usize) -> usize {
        (1..n+1).fold(1, |a, b| a * b)
    }
    let r = &factorial(6);
    assert_eq!(r + &1009, 1729);

    fn f(p: &'static i32) {
        unsafe {
            STASH = p;
        }
    }
}
