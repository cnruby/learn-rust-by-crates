// File: ./examples/kw_fn.rs
// clear && cargo run --example kw_fn_vec_u8 --features ok | bat -l rs
// clear && cargo run --example kw_fn_vec_u8 --features cp | bat -l rs
// clear && cargo run --example kw_fn_vec_u8 --features err

#[cfg(feature = "ok")]
// ANCHOR: feature-ok
fn main() {
    fn fn_borrow(vec_u8s: &Vec<u8>) {
        println!("Inside fn = {:p}", &vec_u8s);
    }

    let vec_instance: Vec<_> = vec![33, 42];
    println!("Before fn = {:p}", &vec_instance);
    fn_borrow(&vec_instance);
    println!("After fn = {:p}", &vec_instance);

    dbg!(vec_instance);
}
// ANCHOR_END: feature-ok

#[cfg(feature = "cp")]
// ANCHOR: feature-cp
fn main() {
    fn fn_borrow(vec_u8s: &Vec<u8>) {
        println!("Inside fn = {:p}", &vec_u8s);
    }
    // ANCHOR: feature-cp-vec
    let mut vec_instance: Vec<u8> = Vec::<u8>::new();
    vec_instance.push(33);
    vec_instance.push(42);
    // ANCHOR_END: feature-cp-vec
    println!("Before fn = {:p}", &vec_instance);
    fn_borrow(&vec_instance);
    println!("After fn = {:p}", &vec_instance);

    dbg!(vec_instance);
}
// ANCHOR_END: feature-cp

#[cfg(feature = "err")]
// error[E0384]
// ANCHOR: feature-err
fn main() {
    fn fn_borrow(vec_u8s: Vec<u8>) {
        println!("Inside fn = {:p}", &vec_u8s);
    }

    let vec_instance: Vec<_> = vec![33, 42];
    println!("Before fn = {:p}", &vec_instance);
    fn_borrow(vec_instance);
    println!("After fn = {:p}", &vec_instance);

    dbg!(vec_instance);
}
// ANCHOR_END: feature-err

#[cfg(all(not(feature = "ok"), not(feature = "err"), not(feature = "cp")))]
fn main() {
    use aide::*;
    hello();
}
