// File: ./examples/mut_string_type.rs
// clear && cargo run --example mut_string_type --features ok | bat -l rs
// clear && cargo run --example mut_string_type --features cp | bat -l rs
// clear && cargo run --example mut_string_type --features err

#![allow(unused_variables)]

#[cfg(feature = "ok")]
fn main() {
    let mut mut_instance = String::from("hello");
    mut_instance.push_str(", world!");
    println!("{}", mut_instance);

    // The variable `mut_instance` begin to move here
    let copy_mut_instance = &mut_instance;
    // The variable `mut_instance` moved here

    // The variable `mut_instance` borrowed here after move
    println!("{}", mut_instance);
}

#[cfg(feature = "err")]
fn main() {
    let mut mut_instance = String::from("hello");
    mut_instance.push_str(", world!");
    println!("{}", mut_instance);

    // The variable `mut_instance` begin to move here
    let copy_mut_instance = mut_instance;
    // The variable `mut_instance` moved here

    // ERROR: The variable `mut_instance` borrowed here after move
    println!("{}", mut_instance);
}

#[cfg(all(not(feature = "ok"), not(feature = "err")))]
fn main() {
    use aide::hello;
    hello();
}
