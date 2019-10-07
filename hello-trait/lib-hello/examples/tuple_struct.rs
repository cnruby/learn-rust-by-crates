mod trait_exerci {
    pub struct TupleStructType (pub u32);
}

// cargo run --example tuple_struct
fn main() {
    let instance = trait_exerci::TupleStructType(0);
    let data = instance.0;
    assert_eq!(0, data);
}