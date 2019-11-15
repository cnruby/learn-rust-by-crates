// File: ./bin-hello/examples/mut_base/mut_count/mod.rs
// clear && cargo run --example mut_base --features ok -- mut_count | bat -l cmd
// clear && cargo run --example mut_base --features err_01
// clear && cargo run --example mut_base --features err_02
// clear && cargo run --example mut_base --features err_03
// clear && cargo run --example mut_base -- mut_count

#![allow(unused_mut)]
#![allow(unused_variables)]

#[cfg(feature = "ok")]
pub fn adjoin() {
    // ANCHOR: feature-ok
    // File: ./bin-hello/examples/mut_base/mut_count/mod.rs
    // #[cfg(feature = "ok")]

    let instance = vec![33u8, 42];
    let first_ref = &instance; // immutable reference
    let second_ref = &instance; // immutable reference
    println!("{:?} {:?}", first_ref, second_ref);

    let mut instance = vec![33, 42u8];
    let first_mut_ref = &instance; // immutable reference
    let second_mut_ref = &instance; // immutable reference
    println!("{:?} {:?}", first_mut_ref, second_mut_ref);

    // ANCHOR_END: feature-ok
}

#[cfg(feature = "err_01")]
// error[E0499]
pub fn adjoin() {
    // ANCHOR: feature-err_01
    // File: ./bin-hello/examples/mut_base/mut_count/mod.rs
    // #[cfg(feature = "err_01")]

    let mut instance = vec![33u8, 42];
    let first_mut_ref = &mut instance; // mutable reference
    let second_mut_ref = &mut instance; // mutable reference
    println!("{:?} {:?}", first_mut_ref, second_mut_ref);

    // ANCHOR_END: feature-err_01
}

#[cfg(feature = "err_02")]
// error[E0502]
pub fn adjoin() {
    // ANCHOR: feature-err_02
    // File: ./bin-hello/examples/mut_base/mut_count/mod.rs
    // #[cfg(feature = "err_02")]

    let mut instance = vec![33, 42u8];
    let first_immut_ref = &instance; // immutable reference
    let second_mut_ref = &mut instance; // mutable reference
    println!("{:?}", first_immut_ref);

    // ANCHOR_END: feature-err_02
}

#[cfg(feature = "err_03")]
// error[E0502]
pub fn adjoin() {
    // ANCHOR: feature-err_03
    // File: ./bin-hello/examples/mut_base/mut_count/mod.rs
    // #[cfg(feature = "err_03")]

    let mut instance = vec![33u8, 42];
    let first_mut_ref = &mut instance; // mutable reference
    let second_immut_ref = &instance; // immutable reference
    println!("{:?}", first_mut_ref);

    // ANCHOR_END: feature-err_03
}

#[cfg(all(
    not(feature = "ok"),
    not(feature = "err_01"),
    not(feature = "err_02"),
    not(feature = "err_03")
))]
pub fn adjoin() {
    use aide::*;
    hello();
}
