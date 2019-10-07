mod trait_exerci {
    pub struct StructType {
        pub data: u32,
    }

    impl StructType {
        pub fn new(data: u32) -> StructType {
            StructType { data: data }
        }
    }
}

// cargo run --example function_instance
fn main() {
    let instance = trait_exerci::StructType::new(0);
    println!("instance.data = {}", instance.data);

    let instance = trait_exerci::StructType{ data:0, };
    println!("instance.data = {}", instance.data);
}