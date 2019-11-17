// File: ./examples/mut_var_sized/string_ref.rs
// clear && cargo run --example mut_var_sized --features ok -- string_ref
// clear && cargo run --example mut_var_sized --features cp -- string_ref
// clear && cargo run --example mut_var_sized --features err_01
// clear && cargo run --example mut_var_sized --features err_02
// clear && cargo run --example mut_var_sized --features err_03

#![allow(unused_variables)]

#[cfg(feature = "ok")]
pub fn adjoin() {
    // ANCHOR: feature-ok
    // File: ./examples/mut_var_sized/string_ref.rs
    // #[cfg(feature = "ok")]

    let mut mut_instance = String::from("Hello");
    mut_instance.push_str(", world");
    println!("1. use mut_instance = {}", mut_instance);

    let immut_ref = &mut_instance;
    println!("1. use immut_ref = {}", immut_ref);

    // The variable `mut_instance` borrowed here after move
    println!("2. use mut_instance = {}", mut_instance);
    println!("2. use immut_ref = {}", immut_ref);

    // ANCHOR_END: feature-ok
}

#[cfg(feature = "err_01")]
pub fn adjoin() {
    // ANCHOR: feature-error_01
    // File: ./examples/mut_var_sized/string_ref.rs
    // ANCHOR = "string_ref-error_01"
    // error[E0382]: borrow of moved value: `mut_instance`

    let mut mut_instance = String::from("hello");
    mut_instance.push_str(", world!");
    println!("1. use mut_instance = {}", mut_instance);

    // The variable `mut_instance` begin to move here
    let copy_mut_instance = mut_instance;
    // The variable `mut_instance` moved here

    // ERROR: The variable `mut_instance` borrowed here after move
    println!("2. use mut_instance = {}", mut_instance); //ERROR

    // ANCHOR_END: feature-error_01
}

#[cfg(feature = "err_02")]
pub fn adjoin() {
    // ANCHOR: feature-error_02
    // File: ./examples/mut_var_sized/string_ref.rs
    // ANCHOR = "string_ref-error_02"
    // error[E0502]: cannot borrow `mut_instance` as mutable because it is also borrowed as immutable

    let mut mut_instance = String::from("Hello");
    mut_instance.push_str(", world");
    println!("1. use mut_instance = {}", mut_instance);

    let immut_ref = &mut_instance;
    println!("1. use immut_ref = {}", immut_ref);

    // The variable `immut_ref` begin to move here
    mut_instance.push_str("!");
    // The variable `immut_ref` moved here
    println!("2. use immut_ref = {}", immut_ref); //ERROR

    // The variable `mut_instance` borrowed here after move
    println!("2. use mut_instance = {}", mut_instance);

    // ANCHOR_END: feature-error_02
}

#[cfg(feature = "cp")]
pub fn adjoin() {
    // ANCHOR: feature-cp
    // File: ./examples/mut_var_sized/string_ref.rs
    // #[cfg(feature = "cp")]

    let mut mut_instance = String::from("Hello");
    mut_instance.push_str(", world");
    println!("1. use mut_instance = {}", mut_instance);

    // The variable `mut_instance` begin to move here
    let mut_ref = &mut mut_instance;
    // The variable `mut_instance` moved here
    println!("1. use mut_ref = {}", mut_ref);
    mut_ref.push_str("!");
    println!("2. use mut_ref = {}", mut_ref);

    mut_instance.make_ascii_uppercase();

    println!("2. use mut_instance = {}", mut_instance);

    // ANCHOR_END: feature-cp
}

#[cfg(feature = "err_03")]
pub fn adjoin() {
    // ANCHOR: feature-error_03
    // File: ./examples/mut_var_sized/string_ref.rs
    // ANCHOR = "string_ref-error_03"
    // error[E0499]: cannot borrow `mut_instance` as mutable more than once at a time

    let mut mut_instance = String::from("Hello");
    mut_instance.push_str(", world");
    println!("1. use mut_instance = {}", mut_instance);

    // The variable `mut_instance` begin to move here
    let mut_ref = &mut mut_instance;
    // The variable `mut_instance` moved here
    println!("1. use mut_ref = {}", mut_ref);
    mut_ref.push_str("!");
    println!("2. use mut_ref = {}", mut_ref);

    // The variable `mut_ref` begin to move here
    mut_instance.make_ascii_uppercase();
    // The variable `mut_ref` moved here

    println!("3. use mut_ref = {}", mut_ref); // ERROR

    // The variable `mut_instance` borrowed here after move
    println!("2. use mut_instance = {}", mut_instance);

    // ANCHOR_END: feature-error_03
}

#[cfg(all(
    not(feature = "ok"),
    not(feature = "cp"),
    not(feature = "err_01"),
    not(feature = "err_02"),
    not(feature = "err_03"),
))]
pub fn adjoin() {
    use aide::hello;
    hello();
}
