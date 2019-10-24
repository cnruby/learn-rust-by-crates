#![allow(unused)]

#[cfg(feature = "ok")]
fn main() {
    fn bar(x: &mut i32) {}
    fn foo(a: &mut i32) {
        bar(a);             // mutable borrow occurs here
        let ref y = a;      // immutable borrow occurs here

        println!("{}", y);  // immutable borrow later used here
    }
}

#[cfg(feature = "err")]
fn main() {
    fn bar(x: &mut i32) {}
    fn foo(a: &mut i32) {
        let ref y = a; // a is borrowed as immutable.
        bar(a); // error: cannot borrow `*a` as mutable because
                //        `a` is also borrowed as immutable
        println!("{}", y);
    }
}

#[cfg(all(not(feature = "ok"), not(feature = "err")))]
fn main() {
    use aide::*;
    hello();
}

// https://doc.rust-lang.org/stable/error-index.html#E0502
