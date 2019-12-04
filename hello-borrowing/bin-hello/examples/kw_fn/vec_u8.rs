// File: ./bin-hello/examples/kw_fn/vec_u8/mod.rs
// clear && cargo run --example kw_fn --features ok -- vec_u8 | bat -l cmd
// clear && cargo run --example kw_fn --features err_01
// clear && cargo run --example kw_fn -- vec_u8

//=======



//=======
#[cfg(feature = "ok")]
pub fn adjoin() {
    // ANCHOR: feature-ok
    // File: ./bin-hello/examples/kw_fn/vec_u8/mod.rs
    // #[cfg(feature = "ok")]

    fn fn_borrow(vec_u8s: &Vec<u8>) {
        println!("Inside fn = {:p}", &vec_u8s);
    }

    let vec_instance: Vec<_> = vec![33, 42];
    println!("Before fn = {:p}", &vec_instance);
    fn_borrow(&vec_instance);
    println!("After fn = {:p}", &vec_instance);

    dbg!(vec_instance);

    // ANCHOR_END: feature-ok
}



//=======
#[cfg(feature = "cp")]
pub fn adjoin() {
    // ANCHOR: feature-cp
    // File: ./bin-hello/examples/kw_fn/vec_u8/mod.rs
    // #[cfg(feature = "cp")]

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

    // ANCHOR_END: feature-cp
}



//=======
#[cfg(feature = "err_01")]
// error[E0384]
pub fn adjoin() {
    // ANCHOR: feature-err
    // File: ./bin-hello/examples/kw_fn/vec_u8/mod.rs
    // #[cfg(feature = "err_01")]

    fn fn_borrow(vec_u8s: Vec<u8>) {
        println!("Inside fn = {:p}", &vec_u8s);
    }

    let vec_instance: Vec<_> = vec![33, 42];
    println!("Before fn = {:p}", &vec_instance);
    fn_borrow(vec_instance);
    println!("After fn = {:p}", &vec_instance);

    dbg!(vec_instance);

    // ANCHOR_END: feature-err
}



//=======
#[cfg(all(not(feature = "ok"), not(feature = "err_01"), not(feature = "cp")))]
pub fn adjoin() {
    use aide::*;
    hello();
}
