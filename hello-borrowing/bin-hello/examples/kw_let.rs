#![allow(unused_variables)]
#![allow(unused_assignments)]

// Run OK:
// cargo run --bin bw -- --file kw_let --mode ok
// cargo run --example kw_let --features 'ok'
// cargo install borrowing_exerci
// bw --file kw_let --mode ok

// Compile-Time Error:
// cargo run --bin bw -- -f kw_let -m error | bat -l rs
// cargo run --bin bw -- -f kw_let | bat -l rs
// cargo run --example kw_let --features 'error'
// cargo install borrowing_exerci
// bw --file kw_let -m error
// bw -f kw_let | bat -l rs

#[cfg(feature = "ok")]
fn main() {
    let x = 7_u8;

    // error[E0384]: cannot assign twice to immutable variable
    // x = 10;
}

// error[E0384]
#[cfg(feature = "error")]
fn main() {
    let x = 7_u8;

    // error[E0384]: cannot assign twice to immutable variable
    x = 10;
}

#[cfg(not(feature = "ok"))]
#[cfg(not(feature = "error"))]
fn main() {
    use aide::hello;
    hello();
}