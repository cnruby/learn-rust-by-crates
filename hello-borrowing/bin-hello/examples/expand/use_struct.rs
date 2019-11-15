// File: ./examples/expand/use_struct.rs
// clear && cargo expand --example expand -- use_struct
// clear && cargo run --example expand -- use_struct

#![allow(unused_variables)]

pub fn adjoin() {
    #[derive(Clone, Copy)]
    struct Struct(u8);

    let a = Struct(42u8);
    let b = a.clone();
    let c = a;

    let _ = a;
}
