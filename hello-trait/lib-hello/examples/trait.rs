#![allow(dead_code)]

mod trait_example {
    pub struct StructType {
        pub data: u32,
    }

    pub trait TraitCanal {
        fn new(data: u32) -> StructType;
    }

    impl TraitCanal for StructType {
        fn new(data: u32) -> StructType {
            StructType { data: data }
        }
    }
}

use trait_example::TraitCanal;

// cargo run --example method_instance
fn main() {
    let instance = trait_example::StructType::new(0);
    println!("instance.data = {}", instance.data);
}