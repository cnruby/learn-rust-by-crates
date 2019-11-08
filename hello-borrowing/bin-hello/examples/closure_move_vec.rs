// File: ./examples/closure_move_vec.rs
// clear && cargo run --example closure_move_vec --features ok | bat -l rs
// clear && cargo run --example closure_move_vec --features err

#![allow(unused_variables)]

#[cfg(feature = "ok")]
// ANCHOR: feature-ok
fn main() {
    let instance = vec![1, 2, 3];

    println!("The variable instance before borrowing: {:?}", instance);

    let ref_instance = &instance;
    let equal_to_val = move |input_var| input_var == ref_instance;

    println!("The variable instance after borrowing: {:?}", instance);

    // use this closure
    let input_instance = vec![1, 2, 3];
    assert!(equal_to_val(&input_instance));
}
// ANCHOR_END: feature-ok

#[cfg(feature = "cp")]
// ANCHOR: feature-cp
fn main() {
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
}
// ANCHOR_END: feature-cp

#[cfg(feature = "err")]
// ANCHOR: feature-err
fn main() {
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
}
// ANCHOR_END: feature-err

#[cfg(all(not(feature = "ok"), not(feature = "err"), not(feature = "cp") ))]
fn main() {
    use aide::hello;
    hello();
}
