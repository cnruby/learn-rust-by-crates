// File: ./examples/expand/struct_str.rs
// clear && cargo run --example expand --features ok -- struct_str
// clear && cargo expand --example expand --features ok -- struct_str
// clear && cargo run --example expand --features cp -- struct_str
// clear && cargo expand --example expand --features cp -- struct_str
// clear && cargo run --example expand --features err_06 -- struct_str
// clear && cargo run --example expand --features err_07 -- struct_str
// clear && cargo run --example expand --features err_08 -- struct_str
// clear && cargo expand --example expand --features err_08 -- struct_str

#![allow(unused_variables)]

#[cfg(feature = "ok")]
pub fn adjoin() {
    // ANCHOR: feature-ok
    // File: ./examples/expand/struct_str.rs
    // #[cfg(feature = "ok")]

    #[derive(Clone, Copy)]
    struct Struct<'cn>(&'cn str);

    let instance: Struct = Struct("Hello");
    let clone_instance = instance.clone();
    let copy_instance = instance;

    // can copy and clone, because derive *Clone*
    // can move, because derive *Copy*,
    // and instance and copy_instance live
    let use_instance = instance;

    // ANCHOR_END: feature-ok
}

#[cfg(feature = "cp")]
pub fn adjoin() {
    // ANCHOR: feature-cp
    // File: ./examples/expand/struct_str.rs
    // #[cfg(feature = "cp")]

    #[derive(Clone)]
    struct Struct<'cn>(&'cn str);

    let instance: Struct = Struct("Hello");
    let clone_instance = instance.clone();
    let copy_instance = instance;

    // can copy and clone, because derive *Clone*
    // but can NOT move, because without derive *Copy*,
    // and instance live not, but copy_instance live
    let use_copy_instance = copy_instance;

    // ANCHOR_END: feature-cp
}

#[cfg(feature = "err_07")]
pub fn adjoin() {
    // ANCHOR: feature-error_01
    // File: ./examples/expand/struct_str.rs
    // ANCHOR = "struct_str-error_01"
    // error[E0382]: use of moved value: `instance`

    #[derive(Clone)]
    struct Struct<'cn>(&'cn str);

    let instance: Struct = Struct("Hello");
    let clone_instance = instance.clone();
    let copy_instance = instance;

    // can copy and clone, because derive *Clone*
    // but can NOT move, because without derive *Copy*,
    // and instance live not
    let use_instance = instance;
    //dbg!(instance.0, clone_instance.0, copy_instance.0);

    // ANCHOR_END: feature-error_01
}

#[cfg(feature = "err_08")]
pub fn adjoin() {
    // ANCHOR: feature-error_02
    // File: ./examples/expand/struct_str.rs
    // ANCHOR = "struct_str-error_02"
    // error[E0382]: use of moved value: `instance`

    #[derive(Clone)]
    struct Struct<'cn>(&'cn str);

    let instance: Struct = Struct("Hello");
    let clone_instance = instance.clone();
    let copy_instance = instance;

    // can copy and clone, because derive *Clone*
    // but can NOT move, because without derive *Copy*,
    // and instance live not, but copy_instance live
    let use_copy_instance = copy_instance;
    let use_instance = instance;

    // ANCHOR_END: feature-error_02
}

#[cfg(feature = "err_09")]
pub fn adjoin() {
    // ANCHOR: feature-error_03
    // File: ./examples/expand/struct_str.rs
    // ANCHOR = "struct_str-error_03"
    // error[E0277]: the trait bound `struct_str::adjoin::Struct<'cn>: std::clone::Clone` is not satisfied

    #[derive(Copy)]
    struct Struct<'cn>(&'cn str);

    let instance: Struct = Struct("Hello");
    let clone_instance = instance.clone();
    let copy_instance = instance;

    // derive *Copy* error, because without derive *Clone*
    let use_instance = instance;
    //dbg!(instance.0, clone_instance.0, copy_instance.0);

    // ANCHOR_END: feature-error_03
}

#[cfg(all(
    not(feature = "ok"),
    not(feature = "cp"),
    not(feature = "err_06"),
    not(feature = "err_07"),
    not(feature = "err_08"),
))]
pub fn adjoin() {
    use aide::*;
    hello();
}
