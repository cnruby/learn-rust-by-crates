mod trait_exerci {
    pub struct ClikeStructType {
        // data: u32,
        pub data: u32,
    }

    pub struct TupleStructType (pub u32);
}

// cargo run --example pub_field
fn main() {
    let instance = trait_exerci::ClikeStructType { data: 0 };
    let data = instance.data;
    assert_eq!(0, data);

    let instance = trait_exerci::TupleStructType(0);
    let data = instance.0;
    assert_eq!(0, data);
}