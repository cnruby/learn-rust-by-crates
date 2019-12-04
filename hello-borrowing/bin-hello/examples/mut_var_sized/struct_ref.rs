// File: ./examples/mut_var_sized/struct_ref.rs
// clear && cargo run --example mut_var_sized --features ok -- struct_ref | bat -l rs
// clear && cargo run --example mut_var_sized --features cp -- struct_ref
// clear && cargo run --example mut_var_sized --features err -- struct_ref

//=======
#![allow(unused_variables)]



//=======
#[cfg(feature = "ok")]
pub fn adjoin() {
    // ANCHOR: feature-ok
    // File: ./examples/mut_var_sized/struct_ref.rs
    // #[cfg(feature = "ok")]

    struct Struct(u8);

    let mut instance = Struct(42u8);
    let ref_mut_instance = &mut instance;
    // How to solve the problem?
    // Go to clone_struct.rs
    ref_mut_instance.0 = 33;
    println!("ref_mut_instance.data = {}", ref_mut_instance.0);
    println!("instance.data = {}", instance.0);

    // ANCHOR_END: feature-ok
}




//=======
// error[E0382]
#[cfg(feature = "err_08")]
pub fn adjoin() {
    // ANCHOR: feature-err_08
    // File: ./examples/mut_var_sized/struct_ref.rs
    // #[cfg(feature = "err_08")]
    //

    struct Struct(u8);

    let mut instance = Struct(42u8);
    let new_instance = instance;
    instance.0 = 33;
    println!("instance.data = {}", instance.0);

    // ANCHOR: feature-err_08
}




//=======
#[cfg(all(not(feature = "ok"), not(feature = "err_08")))]
pub fn adjoin() {
    use aide::hello;
    hello();
}

// https://doc.rust-lang.org/stable/error-index.html#E0382
