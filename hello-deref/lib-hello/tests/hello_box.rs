#[cfg(test)]
mod box_tests {
    use deref_exerci::mod_box;

    #[test]
    fn it_works_for_box() {
        let x: u8 = 5;
        let y: Box<u8> = Box::new(x);
        let z: &u8 = &x;
        assert_eq!(5, x);
        assert_eq!(5, *y);
        assert_eq!(5, *z);

        let x: Box<u8> = Box::new(5);
        let y: &Box<u8> = &x;
        assert_eq!(5, *x);
        assert_eq!(5, **y);
        let y: u8 = *x; // Deref coercion
        assert_eq!(5, *x);
        assert_eq!(5, y);

        let x: Box<u8> = Box::new(5);
        let y: &u8 = &x; // Deref coercion
        assert_eq!(5, *x);
        assert_eq!(5, *y);
    }

    #[test]
    fn it_works_for_box_with_fn() {
        let x: Box<u8> = Box::new(5);
        let y: u8 = mod_box::get_u8(&x); // Deref coercion
        assert_eq!(5, *x);
        assert_eq!(5, y);

        let x: Box<u8> = mod_box::get_box(5);
        let y: &u8 = mod_box::get_ref_box(&x); // Deref coercion
        assert_eq!(5, *x);
        assert_eq!(5, *y);
    }

}
