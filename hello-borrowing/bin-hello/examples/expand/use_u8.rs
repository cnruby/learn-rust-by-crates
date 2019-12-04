// File: ./examples/expand/use_u8.rs
// clear && cargo expand --example expand -- use_u8
// clear && cargo run --example expand -- use_u8

//=======
#![allow(unused_variables)]


//=======
#[cfg(feature = "ok")]
pub fn adjoin() {
    let instance = 42u8;
    let clone_instance = instance.clone();
    let copy_instance = instance;

    let use_instance = instance;
}



//=======
#[cfg(all(
    not(feature = "ok"),
))]
pub fn adjoin() {
    use aide::*;
    hello();
}
