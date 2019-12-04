// File: ./bin-hello/examples/mut_base/mut_ref/mod.rs
// clear && cargo run --example mut_base -- mut_ref


//=======



//=======
#[cfg(feature = "ok")]
pub fn adjoin() {
    // ANCHOR: mut_ref
    // File: ./bin-hello/examples/mut_base/mut_ref/mod.rs

    let mut mut_instance = 1u8;
    println!("1. mut_instance = {}", mut_instance);

    mut_instance = 33;
    println!("2. mut_instance = {}", mut_instance);

    let ref_mut_instance = &mut mut_instance;
    *ref_mut_instance = 42;
    println!("3. mut_instance = {}", mut_instance);

    mut_instance = 100;
    println!("4. mut_instance = {}", mut_instance);

    // ANCHOR_END: mut_ref
}


//=======
#[cfg(all(
    not(feature = "ok"),
))]
pub fn adjoin() {
    use aide::*;
    hello();
}
