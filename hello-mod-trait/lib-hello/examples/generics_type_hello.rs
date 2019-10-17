#![allow(unused_variables)]
struct Struct<T> (T);

// clear && cargo run --example generics_type_hello
fn main() {
    let instance = Struct(0u8);
    let instance = Struct(0u32);
    let instance = Struct('0');
    let instance = Struct(0.0);
    let instance = Struct("0.0");
    let instance = Struct(());
    let instance = Struct([0]);
    let instance = Struct(Struct(0.0f64));
}