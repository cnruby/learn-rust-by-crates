#![allow(unused_variables)]
use std::fmt::Debug;

fn print<T: Debug>(x: T) {
    println!("{:?}", x);
}

// clear && cargo run --example generics_fn_hello
fn main() {
    print(0u8);
    print(0u32);
    print('0');
    print(0.0);
    print("0.0");
    print(());
    print([0]);
}