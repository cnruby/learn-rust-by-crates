#![allow(unused_variables)]
#![allow(unused_assignments)]

#[cfg(feature = "ok")]
// kw_let_mut.rs
// ok
fn main() {
    let mut instance = 42_u8;

    instance = 33;
}

#[cfg(feature = "err")]
// kw_let_mut.rs
// error[E0384]
fn main() {
    let instance = 42_u8;

    // error[E0384]: cannot assign twice to immutable variable
    instance = 33;
}

#[cfg(all(not(feature = "ok"), not(feature = "err")))]
fn main() {
    use aide::*;
    hello();
}
