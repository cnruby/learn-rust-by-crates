// File: ./examples/use_closure.rs
// clear && cargo run --example closure

#![allow(unused_parens)]

pub fn use_closure() {
    // ANCHOR: use_closure
    // File: lib-hello/src/immut/closure/mod.rs
    // Function: use_closure()
    (|__: ()| -> () { __ })(());

    (|input_var: (bool)| -> (bool) { input_var })((true));

    |input_var: (bool)| -> (bool) { input_var }((true));

    let tuple = (true);
    |input_var: (bool)| -> (bool) { input_var }(tuple);

    let tuple = (true);
    let closure_instance = |input_var: (bool)| -> (bool) { input_var };
    closure_instance(tuple);
    // ANCHOR_END: use_closure
}