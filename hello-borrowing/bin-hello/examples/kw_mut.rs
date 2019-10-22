#![allow(unused_mut)]
// cargo run --bin kw_mut

// Run OK:
// cargo run --bin bw -- --file kw_mut --mode ok
// target/debug/bw --file kw_mut --mode ok
// cargo install borrowing_exerci
// bw --file kw_mut --mode ok

// Compile-Time Error:
// cargo run --bin bw -- -f kw_mut -m error
// cargo run --bin bw -- -f kw_mut
// target/debug/bw -f kw_mut -m error
// target/debug/bw -f kw_mut
// cargo install borrowing_exerci
// bw --file kw_mut -m error
// bw -f kw_mut

#[cfg(feature = "ok")]
fn main() {
    let mut berry_instances = vec!["Blackberry", "Strawberry"];

    let ref first_share_ref = &berry_instances; // immutable reference
    let ref second_share_ref = &berry_instances; // immutable reference

    println!("{:?} {:?}", first_share_ref, second_share_ref);
}

// error[E0502]
#[cfg(feature = "error")]
fn main() {
    let mut berry_instances = vec!["Blackberry", "Strawberry"];

    let ref first_share_immut = &berry_instances; // immutable reference
    let ref second_share_mut = &mut berry_instances; // mutable reference

    println!("{:?} {:?}", first_share_immut, second_share_mut);
}

#[cfg(not(feature = "ok"))]
#[cfg(not(feature = "error"))]
fn main() {
    use aide::hello;
    hello();
}

// error[E0499]
/*
fn main() {
    let mut berry_instances = vec!["Blackberry", "Strawberry"];

    let first_share_mut = &mut berry_instances; // mutable reference
    let second_share_mut = &mut berry_instances; // mutable reference

    println!("{:?} {:?}", first_share_mut, second_share_mut);
}
*/

// error[E0502]
/*
fn main() {
    let mut berry_instances = vec!["Blackberry", "Strawberry"];

    let first_share_mut = &mut berry_instances; // mutable reference
    let second_share_immut = &berry_instances; // immutable reference

    println!("{:?} {:?}", first_share_mut, second_share_immut);
}
*/
