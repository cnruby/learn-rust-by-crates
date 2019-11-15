// File: ./examples/use_convert_string_str.rs
// clear && cargo run --example use_convert_string_str | bat -l rs

#![allow(unused_variables)]

fn main() {
    let instance: &str = "Hallo";
    let instance: String = instance.to_owned();
    let instance: &str = instance.as_str();
}
