#[derive(Debug)]
struct AtomType {
    num: u32,
    sym: str,
}

impl std::fmt::Display for AtomType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

struct PeriodicTable {
    atoms: std::collections::HashMap<u32, &'static str>,
}

static atom_data: &str = "A 0 \n\
                          B 1";

// impl PeriodicTable {
//     fn new() -> Self {}
// }

fn main() {
    //println!("{:?}", a.to_string() + "a");
    //let hm = std::collections::HashMap::new();
    for i in atom_data.lines().map(|x| x.split(" ")) {
        println!("{:?}", i);
    }
}
