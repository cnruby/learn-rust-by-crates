#![allow(dead_code)]

mod trait_exerci {
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

use self::trait_exerci::TraitCanal;

// cargo run --example trait
fn main() {
    let instance = trait_exerci::StructType::new(0);
    assert_eq!(0, instance.data);
}