// File ./examples/hola_features.rs
// cargo run --example hola_features --features ok
// cargo run --example hola_features --features err
// cargo run --example hola_features

use hello_borrowing::hola::mod_hola_features;

#[cfg(feature = "ok")]
fn main() {
    mod_hola_features::fn_hola_ok();
}

#[cfg(feature = "err")]
fn main() {
    mod_hola_features::fn_hola_err();
}

#[cfg(all(not(feature = "ok"), not(feature = "err")))]
fn main() {
    mod_hola_features::fn_main();
}
