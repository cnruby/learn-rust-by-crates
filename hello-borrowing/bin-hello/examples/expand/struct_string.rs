// File: ./examples/expand/struct_string.rs
// clear && cargo run --example expand --features ok -- struct_string
// clear && cargo run --example expand --features err_04 -- struct_string
// clear && cargo run --example expand --features err_05 -- struct_string

//=======
#![allow(unused_variables)]


//=======
#[cfg(feature = "ok")]
pub fn adjoin() {
    // ANCHOR: feature-ok
    // File: ./examples/expand/struct_string.rs
    // #[cfg(feature = "ok")]

    #[derive(Clone)]
    struct Struct(String);

    let instance = Struct(String::from("Hello"));
    let clone_instance = instance.clone();
    let copy_instance = instance;

    // can copy and clone, because derive *Clone*
    // can not move, because without derive *Copy*,
    // and instance live not, copy_instance live
    let use_copy_instance = copy_instance;

    // ANCHOR_END: feature-ok
}


//=======
#[cfg(feature = "err_04")]
pub fn adjoin() {
    // ANCHOR: feature-error_01
    // File: ./examples/expand/struct_string.rs
    // ANCHOR = "struct_string-error_01"
    // error[E0204]: the trait `Copy` may not be implemented for this type

    #[derive(Clone, Copy)]
    struct Struct(String);

    let instance = Struct(String::from("Hello"));
    let clone_instance = instance.clone();
    let copy_instance = instance;

    let use_instance = instance;

    // ANCHOR_END: feature-error_01
}


//=======
#[cfg(feature = "err_05")]
pub fn adjoin() {
    // ANCHOR: feature-error_02
    // File: ./examples/expand/struct_string.rs
    // ANCHOR = "struct_string-error_02"
    // error[E0382]: use of moved value: `instance`

    #[derive(Clone)]
    struct Struct(String);

    let instance = Struct(String::from("Hello"));
    let clone_instance = instance.clone();
    let copy_instance = instance;

    // can copy and clone, because derive *Clone*
    // but can NOT move, because without derive *Copy*,
    // and instance live not
    let use_instance = instance;

    // ANCHOR_END: feature-error_02
}


//=======
#[cfg(feature = "err_06")]
pub fn adjoin() {
    // ANCHOR: feature-error_03
    // File: ./examples/expand/struct_string.rs
    // ANCHOR = "struct_string-error_03"
    // error[E0204]: the trait `Copy` may not be implemented for this type

    #[derive(Copy)]
    struct Struct(String);

    let instance = Struct(String::from("Hello"));
    let clone_instance = instance.clone();
    let copy_instance = instance;

    // copy exists, because derive *Clone*
    // but can NOT move, because without derive *Copy*,
    // and instance live not
    let use_instance = instance;

    // ANCHOR_END: feature-error_03
}


//=======
#[cfg(all(
    not(feature = "ok"),
    not(feature = "err_04"),
    not(feature = "err_05"),
    not(feature = "err_06"),
))]
pub fn adjoin() {
    use aide::*;
    hello();
}
