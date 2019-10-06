#![allow(dead_code)]

mod trait_exerci {
    pub struct StructType {
        // data: u32,
        pub data: u32,
    }
}

// cargo run --example pub_field
fn main() {
    trait_exerci::StructType { data: 0 };
}