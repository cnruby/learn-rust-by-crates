mod trait_exerci {
    #[derive(Debug)]
    pub struct StructType {
        pub data: u32,
    }

    pub trait TraitCanal {
        fn new(data: u32) -> StructType;
        fn init() -> StructType { StructType{data:0} }
        fn get_data(&self) -> u32;
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

use self::trait_exerci::TraitCanal;

// cargo run --example trait_with_default_method
fn main() {
    let mut instance = trait_exerci::StructType::new(10);
    instance.set_data(&11);
    println!("new {:?}", instance);

    let mut instance = trait_exerci::StructType::init();
    instance.set_data(&12);
    println!("init {:?}", instance);
}