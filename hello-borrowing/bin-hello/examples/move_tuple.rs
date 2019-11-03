#![allow(unused_variables)]

#[cfg(feature = "ok")]
fn main() {
    #[derive(Debug, Clone, Copy)]
    struct Tuple(u8);
    let v = Tuple(42);
    println!("v is {:p}", &v);

    // clone
    let z = v.clone();
    println!("z is {:p}", &z);
    println!("v is {:p}", &v);

    // copy
    let w = v;
    println!("w is {:p}", &w);
    println!("v is {:p}", &v);
}

#[cfg(feature = "err")]
fn main() {
    #[derive(Debug, Clone)]
    // move occurs because `v` has type `main::Tuple`,
    // which does not implement the `Copy` trait
    struct Tuple(u8);
    let v = Tuple(42);
    println!("v is {:p}", &v);

    // clone
    let z = v.clone();
    println!("z is {:p}", &z);
    println!("v is {:p}", &v);

    // copy
    let w = v; // value moved here
    println!("w is {:p}", &w);
    println!("v is {:p}", &v); // ERROR: value borrowed here after
}

#[cfg(all(not(feature = "ok"), not(feature = "err")))]
fn main() {
    use aide::*;
    hello();
}
