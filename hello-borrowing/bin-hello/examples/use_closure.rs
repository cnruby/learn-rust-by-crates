// File: ./examples/use_closure.rs
// clear && cargo run --example use_closure

#![allow(unused_parens)]
fn main() {
    (|__: ()| -> () { __ })(());

    (|input_var: (bool)| -> (bool) { input_var })((true));

    |input_var: (bool)| -> (bool) { input_var }((true));

    let tuple = (true);
    |input_var: (bool)| -> (bool) { input_var }(tuple);

    let tuple = (true);
    let closure_instance = |input_var: (bool)| -> (bool) { input_var };
    closure_instance(tuple);
}
