//use hello_exercism;
//extern crate hello_exercism;

//cargo test
//cargo test tests::test_hello_world
//cargo test tests::test_hello_world -- --ignored

#[cfg(test)]
mod tests {
    #[test]
    fn test_hello_world() {
        assert_eq!("Hello, World!", hello_exercism::hello());
    }
}

#[test]
fn test_hello_world() {
    assert_eq!("Hello, World!", hello_exercism::hello());
}