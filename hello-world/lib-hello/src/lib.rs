// Rust File: src/lib.rs
pub fn hello() -> &'static str {
    println!("{}", hallo());
    "Hello, World!"
}

fn hallo() -> &'static str {
    "Hallo, Welt!"
}

#[cfg(test)]
#[path = "../private/tests.rs"]
mod tests;

mod tests_identical {
    //use super::*;
    use super::hallo;

    #[test]
    fn es_funktioniert_u() {
        assert_eq!("Hallo, Welt!", hallo());
    }
}

mod tests_same {
    #[test]
    fn es_funktioniert_u() {
        assert_eq!("Hallo, Welt!", super::hallo());
    }
}