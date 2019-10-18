// File: examples/bare_hello.rs
use mod_trait_exerci::mod_bare::StructType;
use mod_trait_exerci::mod_bare::TupleType;

fn get_data_from_struct(instances: [StructType; 2]) {
    let data = instances[0].get_tuple();
    assert_eq!(0, data);
    assert_eq!((0), data);
    assert_eq!(instances[0], instances[1]);
    println!("{:?}", instances[0]);
    println!("{:?}", instances[1]);
}

fn get_data_from_tuple(instances: [TupleType; 2]) {
    let data = instances[0].get_tuple();
    assert_eq!(0, data);
    assert_eq!((0), data);
    assert_eq!(instances[0], instances[1]);
    println!("{:?}", instances[0]);
    println!("{:?}", instances[1]);
}

// clear && cargo run --example bare_hello
fn main() {
    let instances: [StructType; 2] = [Default::default(), StructType{data:0}];
    get_data_from_struct(instances);

    let instances: [TupleType; 2] = [Default::default(), TupleType(0)];
    get_data_from_tuple(instances);    
}
