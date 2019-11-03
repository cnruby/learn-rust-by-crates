use generics_exerci::mod_generics_fn;
use generics_exerci::mod_generics;
use mod_generics::Trait;
use mod_generics::StructType;
use mod_generics::TupleType;

// clear && cargo run --example generics_hello
fn main() {
    let instance: StructType = Default::default();
    println!("1. {}", mod_generics_fn::get_static_all(&instance));
    let instance: TupleType = Default::default();
    println!("2. {}", mod_generics_fn::get_static_all(&instance));

    let instance: StructType = Default::default();
    let instance_box_type: Box<dyn Trait> = Box::new(instance);
    println!("1. {}", mod_generics_fn::get_static_all(&instance_box_type));

/*
    let instance: TupleType = Default::default();
    let instance_box_type: Box<TupleType> = Box::new(instance);
    println!("2. {}", mod_generics_fn::get_static_all(&instance_box_type));
*/
}
