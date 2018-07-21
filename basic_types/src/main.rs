fn build_vector() -> Vec<i16> {
    let mut v: Vec<i16> = Vec::<i16>::new();
    v.push(10i16);
    v.push(20i16);
    v
}

fn build_vector_type_infer() -> Vec<i16> {
    let mut v: Vec<i16> = Vec::<i16>::new();
    v.push(10);
    v.push(20);
    v
}


fn main() {
    let fn1 = |a, b| a*a + b*b;
    let big_val = std::i32::MAX;
    let x = big_val.wrapping_add(1);
    let y = 0b0010_1010;
    
    println!("Hello, world!");
    println!("{:?}", build_vector());
    println!("{:?}", build_vector_type_infer());
    println!("{:?}", fn1(1.1, 1.1));
    println!("{:?}", x);
    println!("{:?}", y);
    assert_eq!( 10_i8 as u16, 10_u16);
    assert_eq!( 10_u16 as i16, 10_i16);
    
    assert_eq!( -1_i16 as i32, -1_i32);
    assert_eq!( 65535_u16 as i32, 65535_i32);

    assert_eq!(2u16.pow(4), 16);
    assert_eq!((-4i32).abs(), 4);
    assert_eq!(0b101101u8.count_ones(), 4);
    
    assert_eq!(5f32.sqrt() * 5f32.sqrt(), 5.);
    assert_eq!(-1.01f64.floor(), -1.0);
    assert!((-1. /std::f32::INFINITY).is_sign_negative());
    println!("{}", (2.0_f64).sqrt());
    println!("{}", f64::sqrt(2.0));
    println!("{}", true as i32);
    println!("{}", true);
    let xx = false;
    if xx == false {
        println!(">>>> false");
    }
    let ch = '\u{CA0}';
    println!("{}_{}", ch, ch);

    println!("{:?}", std::char::from_u32(3u32));

    assert_eq!('*'.is_alphabetic(), false);
    assert_eq!('β'.is_alphabetic(), true);
    assert_eq!('8'.to_digit(10), Some(8));
    assert_eq!('ಠ'.len_utf8(), 3);            
    assert_eq!(std::char::from_digit(2, 10), Some('2'));

    let text = "I see the eigenvalue in thine eye";
    let (head, tail) = text.split_at(21);
    assert_eq!(head, "I see the eigenvalue ");
    assert_eq!(tail, "in thine eye");
}
