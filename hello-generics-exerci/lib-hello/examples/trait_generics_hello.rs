use generics_exerci::mod_generics;
use generics_exerci::mod_generics_fn;
use mod_generics::Trait;
use mod_generics::StructType;
use mod_generics::TupleType;

/*
fn get_data(instance: &StructType) {
    let data = mod_static_fn::get_static_type_ref(instance);
    assert_eq!(0, data);
    println!("{}", data);
    let data = mod_dynamic_fn::get_dynamic_trait_ref(instance);
    assert_eq!(0, data);
    println!("{}", data);
}

fn get_data(instance: &TupleType) {
    let data = mod_static_fn::get_static_type_ref(instance);
    assert_eq!(0, data);
    println!("{}", data);
    let data = mod_dynamic_fn::get_dynamic_trait_ref(instance);
    assert_eq!(0, data);
    println!("{}", data);
}
*/

fn get_data<T>(instance: &T) where T: Trait {
    let data = mod_generics_fn::get_static_all(instance);
    assert_eq!(0, data);
    println!("{}", data);
    let data = mod_generics_fn::get_static_all(instance);
    assert_eq!(0, data);
    println!("{}", data);
}


// clear && cargo run --example trait_generics_hello
fn main() {
    let instance: StructType = Default::default();
    get_data(&instance);

    let instance: TupleType = Default::default();
    get_data(&instance);
}