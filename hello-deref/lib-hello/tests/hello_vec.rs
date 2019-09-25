#[cfg(test)]
mod vec_tests {
    use deref_exerci::mod_vec;

    #[test]
    fn it_works_for_vec() {
        let x: Vec<u8> = vec![1, 2, 3];
        let y: &Vec<u8> = &x;
        assert_eq!(vec![1, 2, 3], x);
        assert_eq!(vec![1, 2, 3], *y);
        let y: &[u8] = &x; // Deref coercion
        assert_eq!(vec![1, 2, 3], x);
        assert_eq!(vec![1, 2, 3], y);
    }

    #[test]
    fn it_works_for_vec_with_fn() {
        let x: Vec<u8> = mod_vec::get_vec(&[1, 2, 3]);
        let y: &Vec<u8> = &x;
        assert_eq!(vec![1, 2, 3], x);
        assert_eq!(vec![1, 2, 3], *y);
        let y: &[u8] = mod_vec::get_slice_vec(&x); // Deref coercion
        assert_eq!(vec![1, 2, 3], x);
        assert_eq!(vec![1, 2, 3], y);
    }

}
