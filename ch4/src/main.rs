use std::rc::Rc;

fn print_padovan() {
    let mut padovan = vec![1 ,1 ,1];
    for i in 3..10 {
        let next = padovan[i-3] + padovan[i-2];
        padovan.push(next);
    }
    println!("P(1..10) = {:?}", padovan);
}

fn main() {
    print_padovan();
    println!("Hello, world!");
    let point = Box::new((0.625, 0.5));
    let label = format!("{:?}", point);
    println!("{:?}", label);

    struct Person {name: String, birth: i32};
    let mut composers = Vec::new();
    composers.push(Person { name: "Palestria".to_string(),
                            birth: 1525 });
    composers.push(Person { name: "Download".to_string(),
                            birth: 1563 });
    composers.push(Person { name: "Lully".to_string(),
                            birth: 1632 });

    for composer in &composers {
        println!("{}, born {}", composer.name, composer.birth);
    }

    let mut v = Vec::new();

    for i in 101 .. 106 {
        v.push(i.to_string());
    }

    // 1. Pop a value off the end of the vector:
    let fifth = v.pop().unwrap();
    assert_eq!(fifth, "105");

    // 2. Move a value out of the middle of the vector,
    // and move the last element into its spot;
    let second = v.swap_remove(1);
    assert_eq!(second, "102");

    // 3. Swap in another value for the one we're taking out;
    let third = std::mem::replace(&mut v[2], "subsitute".to_string());
    assert_eq!(third, "103");

    // Let's see what's left of out vector
    assert_eq!(v, vec!["101", "104", "subsitute"]);

    let vv = vec!["1".to_string(),
                 "2".to_string(),
                 "3".to_string()];

    for mut s in vv {
        s.push('!');
        println!("{}", s);
    }
    struct Person2 { name: Option<String>, birth: i32 }
    let mut composers2 = Vec::new();
    composers2.push(Person2 { name: Some("Palestria".to_string()),
                              birth: 1525});
    let first2_name = std::mem::replace(&mut composers2[0].name, Some("None".to_string()));
    assert_eq!(first2_name, Some("Palestria".to_string()));
    assert_eq!(composers2[0].name, Some("None".to_string()));
    let first3_name = composers2[0].name.take();
    assert_eq!(first3_name, Some("None".to_string()));

    #[derive(Copy, Clone)]
    struct Label { number: u32 };
    fn print(l: Label) { println!("STAMP: {}", l.number); }

    let l = Label { number: 3 };
    print(l);

    let s: Rc<String> = Rc::new("shirataki".to_string());
    let t: Rc<String> = s.clone();
    let u: Rc<String> = t.clone();

    assert!(s.contains("shira"));
    assert_eq!(t.find("taki"), Some(5));
    println!("{} are quite chewy, almost bouncy, but lack flavor", u);
    
}

