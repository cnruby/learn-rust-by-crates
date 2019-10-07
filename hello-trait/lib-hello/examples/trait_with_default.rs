#![allow(dead_code)]

mod xtrait_exerci {
    pub struct StructType {
        pub data: u32,
    }

    pub trait TraitCanal {
        fn new(data: u32) -> StructType;
        fn get_data(&self) -> u32 { 0 }
        fn set_data(&mut self, data: &u32);
    }

    impl TraitCanal for StructType {
        fn new(data: u32) -> StructType {
            StructType { data: data }
        }

        fn get_data(&self) -> u32 {
            self.data
        }

        fn set_data(&mut self, data: &u32) {
            self.data = *data;
        }
    }
}

use self::xtrait_exerci::TraitCanal;

// cargo run --example trait_with_default
fn main() {
    let mut instance = xtrait_exerci::StructType::new(10);
    println!("instance.data = {}", instance.get_data());
    instance.set_data(&11);
    println!("instance.data = {}", instance.get_data());
}