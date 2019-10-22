#![allow(unused)]

// Run OK:
// cargo run --bin bw -- --file kw_mut_fn --mode ok
// target/debug/bw --file kw_mut_fn --mode ok
// cargo run --bin kw_mut_fn --features 'ok'
// cargo install borrowing_exerci
// bw --file kw_mut_fn --mode ok

// Compile-Time Error:
// cargo run --bin bw -- -f kw_mut_fn -m error
// cargo run --bin bw -- -f kw_mut_fn
// target/debug/bw -f kw_mut_fn -m error
// target/debug/bw -f kw_mut_fn
// cargo run --bin kw_mut_fn --features 'error'
// cargo install borrowing_exerci
// bw --file kw_mut_fn -m error
// bw -f kw_mut_fn

#[cfg(feature = "ok")]
fn main() {
    fn bar(x: &mut i32) {}
    fn foo(a: &mut i32) {
        bar(a);             // mutable borrow occurs here
        let ref y = a;      // immutable borrow occurs here

        println!("{}", y);  // immutable borrow later used here
    }
}

#[cfg(feature = "error")]
fn main() {
    fn bar(x: &mut i32) {}
    fn foo(a: &mut i32) {
        let ref y = a;     // a is borrowed as immutable.
        bar(a);            // error: cannot borrow `*a` as mutable because
                           //        `a` is also borrowed as immutable
        println!("{}", y);
    }
}

#[cfg(not(feature = "ok"))]
#[cfg(not(feature = "error"))]
fn main() {
    use aide::hello;
    hello();
}

// https://doc.rust-lang.org/stable/error-index.html#E0502
