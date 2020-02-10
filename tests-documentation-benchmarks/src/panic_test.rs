fn add(x: i32, y: i32) -> i32 {
    x + y
}

#[test]
#[should_panic]
fn this_panics() {
    assert_eq!(add(3, 2), 2);
}
