// File: ./examples/string_type.rs
// clear && cargo run --example string_type_str --features ok | bat -l rs
// clear && cargo run --example string_type_str --features err

#[cfg(feature = "ok")]
// ANCHOR: feature-ok
fn main() {
    let instance = String::from("Hello");
    let raw_instance = instance.as_str();
    println!("raw_instance = {:p}", raw_instance);

    let borrow_instance: &str = &instance;
    println!("borrow_instance = {:p}", borrow_instance);

    println!("{}", instance);
    println!("{}", borrow_instance);
}
// ANCHOR_END: feature-ok

#[cfg(feature = "err")]
// ANCHOR: feature-err
fn main() {
    // move occurs because `instance` has type `std::string::String`,
    // which does not implement the `Copy` trait
    let instance = String::from("hello");

    // The variable `instance` begin to move here
    let copy_instance = instance;
    // The variable `instance` moved here

    // ERROR: The variable `instance` borrowed here after move
    println!("{}", instance);
    println!("{}", copy_instance);
}
// ANCHOR_END: feature-err

#[cfg(all(not(feature = "ok"), not(feature = "err")))]
fn main() {
    use aide::hello;
    hello();
}
