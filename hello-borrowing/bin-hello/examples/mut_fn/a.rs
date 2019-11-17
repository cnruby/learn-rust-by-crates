// File: ./examples/mut_fn/double_refs.rs
// clear && cargo run --example mut_fn --features ok -- double_refs | bat -l rs
// clear && cargo run --example mut_fn --features err_01
// clear && cargo run --example mut_fn --features err_02

#![allow(unused)]

#[cfg(feature = "ok")]
pub fn adjoin() {
    // ANCHOR: feature-ok
    // File: ./examples/mut_fn/double_refs.rs
    // #[cfg(feature = "ok")]

    fn bar(x: &mut i32) {}
    fn foo(mut_ref: &mut i32) {
        bar(mut_ref); // mutable borrow occurs here
        let immut_ref_ref = &mut_ref; // immutable borrow occurs here

        println!("{}", immut_ref_ref); // immutable borrow later used here
    }

    // ANCHOR_END: feature-ok
}



#[cfg(feature = "err_01")]
pub fn adjoin() {
    // ANCHOR: feature-error_01
    // File: ./examples/mut_var_sized/string_refs.rs
    // ANCHOR = "string_refs-error_01"
    // 

    fn bar(x: &mut i32) {}
    fn foo(mut_ref: &mut i32) {
        let immut_ref_ref = &mut_ref; // a is borrowed as immutable.
        bar(a); // error: cannot borrow `*a` as mutable because
                //        `a` is also borrowed as immutable
        println!("{}", immut_ref_ref);
    }

    // ANCHOR_END: feature-err_01
}



#[cfg(feature = "err_02")]
pub fn adjoin() {
    fn bar(x: &mut i32) {}
    fn foo(mut_ref: &mut i32) {
        let immut_ref_ref = &mut_ref; // a is borrowed as immutable.
        bar(a); // error: cannot borrow `*a` as mutable because
                //        `a` is also borrowed as immutable
        println!("{}", immut_ref_ref);
    }
}



#[cfg(all(
    not(feature = "ok"),
    not(feature = "err_01"),
    not(feature = "err_02"),
))]
pub fn adjoin() {
    use aide::*;
    hello();
}

// https://doc.rust-lang.org/stable/error-index.html#E0502
