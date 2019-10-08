mod trait_exerci {
    pub struct StructType {
        data: u32,
    }

    impl StructType {
        pub fn new(data: u32) -> StructType {
            StructType { data: data }
        }

        pub fn get_data(&self) -> u32 {
            self.data
        }

        pub fn set_data(&mut self, data: &u32) {
            self.data = *data;
        }
    }
}

// cargo run --example function_methods
fn main() {
    let instance = trait_exerci::StructType::new(0);
    let data = instance.get_data();
    println!("{0: <20} = {1: <20}", "instance data", data);

    let mut instance = trait_exerci::StructType::new(0);
    instance.set_data(&10);
    let data = instance.get_data();
    println!("{0: <20} = {1: <20}", "mut instance data", data);

    let data = trait_exerci::StructType::new(20).get_data();
    println!("{0: <20} = {1: <20}", "data", data);
}