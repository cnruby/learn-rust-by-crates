fn main () {
    println!("{}",hello_exercism::hello());
}

#[test]
fn test_hello_world() {
    assert_eq!("Hello, World!", hello_exercism::hello());
}