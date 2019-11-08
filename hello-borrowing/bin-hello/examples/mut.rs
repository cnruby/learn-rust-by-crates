// File: ./examples/mut.rs
// clear && cargo run --example mut --features ok | bat -l rs
// clear && cargo run --example mut --features cp
// clear && cargo run --example mut --features err

#![allow(unused_mut)]

#[cfg(feature = "ok")]
fn main() {
    let instance = vec![33, 42];
    let ref first_ref = &instance; // immutable reference
    let ref second_ref = &instance; // immutable reference
    println!("{:?} {:?}", first_ref, second_ref);

    let mut instance = vec![33, 42];
    let ref first_mut_ref = &instance; // immutable reference
    let ref second_mut_ref = &instance; // immutable reference
    println!("{:?} {:?}", first_mut_ref, second_mut_ref);
}

#[cfg(feature = "cp")]
// error[E0499]
fn main() {
    let mut instance = vec![33, 42];
    let first_mut_ref = &mut instance; // mutable reference
    let second_mut_ref = &mut instance; // mutable reference
    println!("{:?} {:?}", first_mut_ref, second_mut_ref);
}

#[cfg(feature = "err")]
// error[E0502]
fn main() {
    let mut instance = vec![33, 42];
    let ref first_immut_ref = &instance; // immutable reference
    let ref second_mut_ref = &mut instance; // mutable reference
    println!("{:?} {:?}", first_immut_ref, second_mut_ref);

    let mut instance = vec![33, 42];
    let first_mut_ref = &mut instance; // mutable reference
    let second_immut_ref = &instance; // immutable reference
    println!("{:?} {:?}", first_mut_ref, second_immut_ref);
}

#[cfg(all(not(feature = "ok"), not(feature = "err"), not(feature = "cp") ))]
fn main() {
    use aide::*;
    hello();
}