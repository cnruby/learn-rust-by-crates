// File: examples/trait_fn_hello.rs
// clear && cargo run --example trait_fn_hello
use mod_trait_exerci::mod_static_fn;
use mod_trait_exerci::mod_dynamic_fn;
use mod_trait_exerci::mod_trait;
use mod_trait::StructType;
use mod_trait::TupleType;

fn get_data_from_struct(instance: &StructType) {
    let data = mod_static_fn::get_static_type_ref(instance);
    assert_eq!(0, data);
    assert_eq!((0), data);
    let data = mod_dynamic_fn::get_dynamic_trait_ref(instance);
    assert_eq!(0, data);
    assert_eq!((0), data);
}

fn get_data_from_tuple(instance: &TupleType) {
    let data = mod_static_fn::get_static_type_ref(instance);
    assert_eq!(0, data);
    assert_eq!((0), data);
    let data = mod_dynamic_fn::get_dynamic_trait_ref(instance);
    assert_eq!(0, data);
    assert_eq!((0), data);
}

/*
fn get_data<T>(instance: &T) where T: Trait {
    let data = mod_generics_fn::get_static_all(instance);
    assert_eq!(0, data);
    println!("{}", data);
    let data = mod_generics_fn::get_static_all(instance);
    assert_eq!(0, data);
    println!("{}", data);
}
*/

// clear && cargo run --example trait_fn_hello
fn main() {
    let instance: StructType = Default::default();
    get_data_from_struct(&instance);

    let instance: TupleType = Default::default();
    get_data_from_tuple(&instance);
}