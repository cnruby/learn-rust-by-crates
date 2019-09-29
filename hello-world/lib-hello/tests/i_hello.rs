// Rust File: tests/i_hello.rs
#[test]
fn it_works_with_extern() {
    assert_eq!("Hello, World!", i_crate::hello());
}

#[test]
fn it_works_with_the_crate_and_extern() {
    assert_eq!(hello_exercism::hello(), i_crate::hello());
}
