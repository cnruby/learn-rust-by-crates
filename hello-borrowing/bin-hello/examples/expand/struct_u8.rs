// File: ./examples/expand/struct_u8.rs
// clear && cargo run --example expand --features ok -- struct_u8
// clear && cargo expand --example expand --features ok -- struct_u8
// clear && cargo run --example expand --features cp -- struct_u8
// clear && cargo expand --example expand --features cp -- struct_u8
// clear && cargo run --example expand --features err_01 -- struct_u8
// clear && cargo run --example expand --features err_02 -- struct_u8
// clear && cargo run --example expand --features err_03 -- struct_u8
// clear && cargo expand --example expand --features err_03 -- struct_u8

//=======
#![allow(unused_variables)]


//=======
#[cfg(feature = "ok")]
pub fn adjoin() {
    // ANCHOR: feature-ok
    // File: ./examples/expand/struct_u8.rs
    // #[cfg(feature = "ok")]

    #[derive(Clone, Copy)]
    struct Struct(u8);

    let instance: Struct = Struct(42u8);
    let clone_instance = instance.clone();
    let copy_instance = instance;

    // can copy and clone, because derive *Clone*
    // can move, because derive *Copy*,
    // and instance and copy_instance live
    let use_instance = instance;

    // ANCHOR_END: feature-ok
}


//=======
#[cfg(feature = "cp")]
pub fn adjoin() {
    // ANCHOR: feature-cp
    // File: ./examples/expand/struct_u8.rs
    // #[cfg(feature = "cp")]

    #[derive(Clone)]
    struct Struct(u8);

    let instance = Struct(42u8);
    let clone_instance = instance.clone();
    let copy_instance = instance;

    // can copy and clone, because derive *Clone*
    // but can NOT move, because without derive *Copy*,
    // and instance live not, but copy_instance live
    let use_copy_instance = copy_instance;

    // ANCHOR_END: feature-cp
}


//=======
#[cfg(feature = "err_01")]
pub fn adjoin() {
    // ANCHOR: feature-error_01
    // File: ./examples/expand/struct_u8.rs
    // ANCHOR = "struct_u8_error_01"
    // error[E0382]: use of moved value: `instance`

    #[derive(Clone)]
    struct Struct(u8);

    let instance = Struct(42u8);
    let clone_instance = instance.clone();
    let copy_instance = instance;

    // can copy and clone, because derive *Clone*
    // but can NOT move, because without derive *Copy*,
    // and instance live not
    let use_instance = instance;

    // ANCHOR_END: feature-error_01
}


//=======
#[cfg(feature = "err_02")]
pub fn adjoin() {
    // ANCHOR: feature-error_02
    // File: ./examples/expand/struct_u8.rs
    // ANCHOR = "struct_u8_error_02"
    // error[E0382]: use of moved value: `instance`

    #[derive(Clone)]
    struct Struct(u8);

    let instance = Struct(42u8);
    let clone_instance = instance.clone();
    let copy_instance = instance;

    // can copy and clone, because derive *Clone*
    // but can NOT move, because without derive *Copy*,
    // and instance live not, but copy_instance live
    let use_copy_instance = copy_instance;
    let use_instance = instance;

    // ANCHOR_END: feature-error_02
}


//=======
#[cfg(feature = "err_03")]
pub fn adjoin() {
    // ANCHOR: feature-error_03
    // File: ./examples/expand/struct_u8.rs
    // ANCHOR = "struct_u8_error_03"
    // error[E0277]: the trait bound `struct_u8::adjoin::Struct: std::clone::Clone` is not satisfied

    #[derive(Copy)]
    struct Struct(u8);

    let instance = Struct(42u8);
    let clone_instance = instance.clone();
    let copy_instance = instance;

    // derive *Copy* error, because without derive *Clone*
    let use_instance = instance;

    // ANCHOR_END: feature-error_03
}


//=======
#[cfg(feature = "err_10")]
pub fn adjoin() {
    // ANCHOR: feature-error_04
    // File: ./examples/expand/struct_u8.rs
    // ANCHOR = "struct_u8_error_04"
    // error[E0382]: use of moved value: `instance`

    struct Struct(u8);

    let instance = Struct(42u8);
    let copy_instance = instance;

    let use_instance = instance;

    // ANCHOR_END: feature-error_04
}


//=======
#[cfg(all(
    not(feature = "ok"),
    not(feature = "cp"),
    not(feature = "err_01"),
    not(feature = "err_02"),
    not(feature = "err_03"),
    not(feature = "err_10"),
))]
pub fn adjoin() {
    use aide::*;
    hello();
}
