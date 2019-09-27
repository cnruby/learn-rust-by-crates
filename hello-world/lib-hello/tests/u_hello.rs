// Rust File: tests/u_hello.rs
#[cfg(test)]
mod tests {
    #[test]
    fn it_works_u() {
        assert_eq!("Hello, World!", hello_exercism::hello());
    }
}
