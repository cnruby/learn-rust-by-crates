#![allow(unused_variables)]

#[cfg(feature = "ok")]
fn main() {
    let instance = String::from("hello");
    println!("{}", instance);

    // The variable `instance` begin to move here
    let copy_instance = instance;
    // The variable `instance` moved here

    // The variable `instance` borrowed here after move
    // println!("{}", instance);
}

#[cfg(feature = "err")]
fn main() {
    // move occurs because `instance` has type `std::string::String`,
    // which does not implement the `Copy` trait
    let instance = String::from("hello");
    println!("{}", instance);

    // The variable `instance` begin to move here
    let copy_instance = instance;
    // The variable `instance` moved here

    // ERROR: The variable `instance` borrowed here after move
    println!("{}", instance);
}

#[cfg(all(not(feature = "ok"), not(feature = "err")))]
fn main() {
    use aide::hello;
    hello();
}
