// File: ./bin-hello/examples/string_type/string_str/mod.rs
// clear && cargo run --example string_type --features ok -- string_str | bat -l cmd
// clear && cargo run --example string_type --features err_01
// clear && cargo run --example string_type -- string_str

//=======


//=======
#[cfg(feature = "ok")]
pub fn adjoin() {
    // ANCHOR: feature-ok
    // File: ./bin-hello/examples/string_type/string_str/mod.rs
    // #[cfg(feature = "ok")]

    let instance = String::from("Hello");
    let raw_instance = instance.as_str();
    println!("raw_instance = {:p}", raw_instance);

    let borrow_instance: &str = &instance;
    println!("borrow_instance = {:p}", borrow_instance);

    println!("{}", instance);
    println!("{}", borrow_instance);

    // ANCHOR_END: feature-ok
}



//=======
#[cfg(feature = "err_01")]
pub fn adjoin() {
    // ANCHOR: feature-err_01
    // File: ./bin-hello/examples/string_type/string_str/mod.rs
    // #[cfg(feature = "err_01")]

    // move occurs because `instance` has type `std::string::String`,
    // which does not implement the `Copy` trait
    let instance = String::from("hello");

    // The variable `instance` begin to move here
    let copy_instance = instance;
    // The variable `instance` moved here

    // ERROR: The variable `instance` borrowed here after move
    println!("{}", instance);
    println!("{}", copy_instance);

    // ANCHOR_END: feature-err_01
}




//=======
#[cfg(all(not(feature = "ok"), not(feature = "err_01")))]
pub fn adjoin() {
    use aide::hello;
    hello();
}
