struct MyStack {
    len: u64,
    item: i64,
    previous: Box<Option<MyStack>>,
}

impl MyStack {
    fn new(item: i64) -> Self {
        MyStack {
            len: 1,
            item: item,
            previous: Box::new(None),
        }
    }
    fn append(self, item: i64) -> Self {
        MyStack {
            len: self.len + 1,
            item: item,
            previous: Box::new(Some(self)),
        }
    }
    fn pop(mut self) -> i64 {
        let tmp = self.item;
        self = self.previous.unwrap();
        tmp
    }
}

impl Iterator for MyStack {
    type Item = MyStack;
    fn next(&mut self) -> Option<Self::Item> {
        let curr = self;
        self = self.previous.unwrap();
        Some(*curr)
    }
}

impl std::fmt::Display for MyStack {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("1, 2, 3")
    }
}

fn main() {
    let list = MyStack::new(3);
    let list = list.append(5);
    let list = list.append(20);
    println!("{}", list);
}
