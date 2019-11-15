// File: ./examples/closure_string.rs
// clear && cargo run --example closure_string --features ok | bat -l rs
// clear && cargo run --example closure_string --features err

#[cfg(feature = "ok")]
// ANCHOR: feature-ok
/// File: ./bin-hello/examples/closure_string.rs
fn main() {
    let string_instance: String = "Hello".to_string();

    println!("Before fn = {:p}", &string_instance);
    let closure_instance = |hello: &str| {
        println!("{} Friend!", hello);
        println!("Inside fn = {:p}", &hello);
    };
    closure_instance(&string_instance);
    println!("After fn = {:p}", &string_instance);

    println!("{} World!", string_instance);
}
// ANCHOR_END: feature-ok

#[cfg(feature = "err")]
// ANCHOR: feature-err
/// File: ./bin-hello/examples/closure_string.rs
fn main() {
    let string_instance: String = "Hello".to_string();

    let closure_instance = |hello: String| println!("{} Friend!", hello);
    closure_instance(string_instance);

    println!("{} World!", string_instance);
}
// ANCHOR_END: feature-err

#[cfg(all(not(feature = "ok"), not(feature = "err")))]
fn main() {
    use aide::hello;
    hello();
}
