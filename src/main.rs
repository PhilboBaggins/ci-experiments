fn main() {
    println!("Hello, world!");
}

#[test]
fn five_equals_five() {
    assert!(5 == 5);
}

#[test]
#[should_panic(expected = "assertion failed")]
fn it_panics() {
    assert!(false);
}
