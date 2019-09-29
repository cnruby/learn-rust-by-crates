// Rust File: ./integration_tests/i_hello.rs
use crate as hello_exercism;

#[test]
fn it_works_with_only_extern() {
    assert_eq!("Hello, World!", i_crate::hello());
}

#[test]
fn it_works_with_the_crate_and_extern() {
    assert_eq!(hello_exercism::hello(), i_crate::hello());
}
