// File: ./examples/closure_string.rs
// clear && cargo run --example closure_string --features ok
// clear && cargo run --example closure_string --features err

#[cfg(feature = "ok")]
fn main() {
    let string_instance: String = "Hello".to_string();

    let greet_friend = |str: &str| println!("{} Friend", str);
    greet_friend(&string_instance);

    println!("{} World!", string_instance);
}

#[cfg(feature = "err")]
fn main() {
    let string_instance: String = "hello".to_string();

    let greet_friend = |string: String| println!("{} Friend", string);
    greet_friend(string_instance);

    println!("{} World!", string_instance);
}

#[cfg(all(not(feature = "ok"), not(feature = "err")))]
fn main() {
    use aide::hello;
    hello();
}
