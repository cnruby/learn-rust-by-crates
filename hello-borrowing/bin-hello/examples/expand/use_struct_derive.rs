// File: ./examples/use_struct_derive.rs
// clear && cargo expand --example use_struct_derive

#![allow(unused_variables)]

fn main() {
    //use std::ptr::eq;

    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Struct(u8);

    let a = Struct(0);
    let b = a;
    let c = a.clone();

    dbg!(a, b, c);
    println!("{:?} {:?} {:?}", a, b, c);
    println!("{:p} {:p} {:p}", &a, &b, &c);

    assert_eq!(a, b);
    assert_eq!(a, c);

    assert!(!eq(&a, &b));
    assert!(!eq(&a, &c));
}
