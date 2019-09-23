#[cfg(test)]
mod other_tests {

    #[test]
    fn it_works_for_u8() {
        let x: u8 = 5;
        let y: &u8 = &x;
        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    #[test]
    fn it_works_for_str() {
        let x: &str = "Hello";
        let y: &&str = &x;
        assert_eq!("Hello", x);
        assert_eq!("Hello", *y);
    }

    #[test]
    fn it_works_for_char() {
        let x: char = '?';
        let y: &char = &x;
        assert_eq!('?', x);
        assert_eq!('?', *y);
    }

    #[test]
    fn it_works_for_pointer() {
        let n = 5;
        println!("{:p}", &n);
        let b = Box::new(n);
        println!("{:p}", b); // notice: "&" exists not before b
    }

}
