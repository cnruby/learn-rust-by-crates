#![allow(unused_variables)]
#![allow(unused_assignments)]

#[cfg(feature = "ok")]
fn main() {
    let mut instance = 42_u8;

    instance = 33;
}

// error[E0384]
#[cfg(feature = "err")]
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
