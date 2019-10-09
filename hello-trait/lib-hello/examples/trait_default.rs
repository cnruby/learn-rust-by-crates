#![allow(dead_code)]

mod trait_exerci {
    #[derive(Default, Debug)]
    pub struct StructType {
        pub data: u32,
    }
}

// cargo run --example trait_default
fn main() {
    let instance: trait_exerci::StructType = Default::default();
    println!("{0: <20} = {1}", "instance.data", instance.data);
    println!("{0: <20} = {1:?}", "instance", instance);
}