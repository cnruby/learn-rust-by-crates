// File: ./bin-hello/examples/closure/move_vec/mod.rs
// clear && cargo run --example closure --features ok -- move_vec | bat -l cmd
// clear && cargo run --example closure --features err_03
// clear && cargo run --example closure --features err_04
// clear && cargo run --example closure -- move_vec

//=======
#![allow(unused_variables)]



//=======
#[cfg(feature = "ok")]
pub fn adjoin() {
    // ANCHOR: feature-ok
    // File: ./bin-hello/examples/closure/move_vec/mod.rs
    // #[cfg(feature = "ok")]

    let instance = vec![1, 2, 3];

    println!("The variable instance before borrowing: {:?}", instance);

    let ref_instance = &instance;
    let equal_to_val = move |input_var| input_var == ref_instance;

    println!("The variable instance after borrowing: {:?}", instance);

    // use this closure
    let input_instance = vec![1, 2, 3];
    assert!(equal_to_val(&input_instance));

    // ANCHOR_END: feature-ok
}

//=======
#[cfg(feature = "err_03")]
pub fn adjoin() {
    // ANCHOR: feature-err_03
    // File: ./bin-hello/examples/closure/move_vec/mod.rs
    // #[cfg(feature = "err_03")]

    let instance = vec![1, 2, 3];
    println!("The variable instance before borrowing: {:?}", instance);

    // The variable instance begin to move here
    let equal_to_val = move |input_var| input_var == &instance;
    // The variable instance moved here

    // The variable instance borrowed here after move
    println!("The variable instance after borrowing: {:?}", instance);

    // use this closure
    let input_instance = vec![1, 2, 3];
    assert!(equal_to_val(&input_instance));

    // ANCHOR_END: feature-err_03
}

//=======
#[cfg(feature = "err_04")]
pub fn adjoin() {
    // ANCHOR: feature-err_04
    // File: ./bin-hello/examples/closure/move_vec/mod.rs
    // #[cfg(feature = "err_04")]

    let instance = vec![1, 2, 3];

    println!("The variable instance before borrowing: {:?}", instance);
    // The variable instance begin to move here
    //let equal_to_val = move |input_var| {input_var == instance};
    let equal_to_val = move |input_var| input_var == instance;
    // The variable instance moved here

    // The variable instance borrowed here after move
    println!("The variable instance after borrowing: {:?}", instance);

    // use this closure
    let input_instance = vec![1, 2, 3];
    assert!(equal_to_val(input_instance));

    // ANCHOR_END: feature-err_04
}

//=======
#[cfg(all(not(feature = "ok"), not(feature = "err_03"), not(feature = "err_04")))]
pub fn adjoin() {
    use aide::hello;
    hello();
}