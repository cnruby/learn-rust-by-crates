#![allow(unused_variables)]

#[cfg(feature = "ok")]
fn main() {
    let v: Vec<u8> = vec![1, 2, 3];
    println!("v is {:p}", &v);

    // clone
    // *cloned* the variable v, rendering v usable.
    let z = v.clone();
    println!("z is {:p}", &z);
    println!("v is {:p}", &v);

    // move / copy
    // *moved* the variable v, rendering v unusable.
    let w = v;
    println!("w is {:p}", &w);
    //println!("v is {:p}", &v);            // ERROR
}

#[cfg(feature = "err")]
fn main() {
    // move occurs because `v` has type `std::vec::Vec<u8>`,
    // which does not implement the `Copy` trait
    let v: Vec<u8> = vec![1, 2, 3];
    println!("v is {:p}", &v);

    // clone
    let z = v.clone();
    println!("z is {:p}", &z);
    println!("v is {:p}", &v);

    // move / copy
    // *moved* the variable v, rendering v unusable.
    let w = v; // value moved here
    println!("w is {:p}", &w);
    println!("v is {:p}", &v); // ERROR: value borrowed here after move
}

#[cfg(all(not(feature = "ok"), not(feature = "err")))]
fn main() {
    use aide::*;
    hello();
}
