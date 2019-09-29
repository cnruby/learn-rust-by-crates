// Rust File: examples/i_hello.rs
fn main() {
    println!("{}", i_crate::hello());
    assert_eq!(hello_exercism::hello(), i_crate::hello());
}