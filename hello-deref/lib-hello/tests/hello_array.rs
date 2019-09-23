#[cfg(test)]
mod array_tests {
    use deref_exerci::mod_array;

    #[test]
    fn it_works_for_array() {
        let x: [u8; 3] = [1, 2, 3];
        let y: &[u8; 3] = &x;
        assert_eq!([1, 2, 3], x);
        assert_eq!([1, 2, 3], *y);
        let y: &[u8] = &x; // Deref coercion
        assert_eq!([1, 2, 3], x);
        assert_eq!([1, 2, 3], y);
    }

    #[test]
    fn it_works_for_array_with_fn() {
        let x: [u8; 3] = mod_array::get_array(&[1, 2, 3]);
        let y: &[u8; 3] = &x;
        assert_eq!([1, 2, 3], x);
        assert_eq!([1, 2, 3], *y);
        let y: &[u8] = mod_array::get_slice_array(&x); // Deref coercion
        assert_eq!([1, 2, 3], x);
        assert_eq!([1, 2, 3], y);
    }
}
