use std::ptr::eq;

fn main() {
    let a = 42u8;
    let b = a;
    println!("{:?} {:?}", a, b);
    println!("{:p} {:p}", &a, &b);

    assert_eq!(a, b);
    assert!(!eq(&a, &b));
}