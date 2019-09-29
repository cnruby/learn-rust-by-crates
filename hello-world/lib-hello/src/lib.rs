// Rust File: src/lib.rs
pub fn hello() -> &'static str {
    println!("{}", hallo());
    "Hello, World!"
}

fn hallo() -> &'static str {
    "Hallo, Welt!"
}

// BEGIN: unit tests for private code
// code 1
#[cfg(test)]
#[path = "./private_tests/owned_hello.rs"]
mod owned_hello;

// code 2
#[cfg(test)]
#[path = "./private_tests/mod.rs"]
mod private_tests;

// code 3
#[cfg(test)]
mod private_tests_with_use {
    use super::*;
    //use super::hallo;

    #[test]
    fn it_works_at_private() {
        assert_eq!("Hallo, Welt!", hallo());
    }
}

// code 4
#[cfg(test)]
mod private_tests_without_use {
    #[test]
    fn it_works_at_private() {
        assert_eq!("Hallo, Welt!", super::hallo());
    }
}
// END unit tests for private code 

// BEGIN: integration tests
#[cfg(test)]
#[path = "./integration_tests/i_hello.rs"]
mod i_hello;

#[cfg(test)]
#[path = "./integration_tests/mod.rs"]
mod integration_tests;
// END: integration tests