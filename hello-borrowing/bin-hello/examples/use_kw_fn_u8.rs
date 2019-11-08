// File: ./examples/use_kw_fn_u8.rs
// clear && cargo run --example use_kw_fn_u8 --features ok | bat -l rs
// clear && cargo run --example use_kw_fn_u8 --features cp | bat -l rs

#[cfg(feature = "ok")]
// ANCHOR: features-ok
fn main() {
    fn fn_borrow(num: u8) {
        println!("inside fn = {:p}", &num);
    }

    let num = 42;
    println!("Before fn = {:p}", &num);
    fn_borrow(num);
    println!("After fn = {:p}", &num);

    dbg!(num);
}
// ANCHOR_END: features-ok

// ANCHOR: features-cp
#[cfg(feature = "cp")]
fn main() {
    fn fn_borrow(_: u8) {}

    let num = 42;
    fn_borrow(num);

    dbg!(num);
}
// ANCHOR_END: features-cp

#[cfg(all(not(feature = "ok"), not(feature = "cp")))]
fn main() {
    use aide::hello;
    hello();
}
