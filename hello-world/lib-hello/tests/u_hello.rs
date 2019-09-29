// Rust File: tests/u_hello.rs
#[cfg(test)]
mod tests {
    #[test]
    fn it_works_at_uint() {
        assert_eq!("Hello, World!", hello_exercism::hello());
    }
}