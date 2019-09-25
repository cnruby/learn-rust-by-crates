#[cfg(test)]
mod string_tests {
    use deref_exerci::mod_string;

    #[test]
    fn it_works_for_string() {
        let x: String = String::from("Hello");
        let y: &String = &x;
        assert_eq!("Hello", x);
        assert_eq!("Hello", &(*x)[..]);
        assert_eq!("Hello", *y);
        let y: &str = &x; // Deref coercion
        assert_eq!("Hello", x);
        assert_eq!("Hello", y);
    }

    #[test]
    fn it_works_for_string_wthi_fn() {
        let x: String = mod_string::get_string("Hello");
        let y: &String = &x;
        assert_eq!("Hello", x);
        assert_eq!("Hello", &(*x)[..]);
        assert_eq!("Hello", *y);
        let y = mod_string::get_slice_str(&x);
        assert_eq!("Hello", x);
        assert_eq!("Hello", y);
    }
}
