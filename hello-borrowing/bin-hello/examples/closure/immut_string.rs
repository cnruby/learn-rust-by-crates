// File: ./bin-hello/examples/closure/immut_string/mod.rs
// clear && cargo run --example closure --features ok -- immut_string | bat -l cmd
// clear && cargo run --example closure --features err_01
// clear && cargo run --example closure -- immut_string

//=======


//=======
#[cfg(feature = "ok")]
pub fn adjoin() {
    // ANCHOR: feature-ok
    // File: ./bin-hello/examples/closure/immut_string/mod.rs
    // #[cfg(feature = "ok")]

    let string_instance: String = "Hello".to_string();

    println!("Before fn = {:p}", &string_instance);
    let closure_instance = |hello: &str| {
        println!("{} Friend!", hello);
        println!("Inside fn = {:p}", &hello);
    };
    closure_instance(&string_instance);
    println!("After fn = {:p}", &string_instance);

    println!("{} World!", string_instance);

    // ANCHOR_END: feature-ok
}

//=======
#[cfg(feature = "err_01")]
pub fn adjoin() {
    // ANCHOR: feature-err
    // File: ./bin-hello/examples/closure/immut_string/mod.rs
    // #[cfg(feature = "err_01")]

    let string_instance: String = "Hello".to_string();

    let closure_instance = |hello: String| println!("{} Friend!", hello);
    closure_instance(string_instance);

    println!("{} World!", string_instance);

    // ANCHOR_END: feature-err
}


//=======
#[cfg(all(not(feature = "ok"), not(feature = "err_01")))]
pub fn adjoin() {
    use aide::hello;
    hello();
}
