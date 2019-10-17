// File: examples/box_dynamic_hello.rs
use mod_trait_exerci::mod_dynamic_fn;
use mod_trait_exerci::mod_trait::StructType;
use mod_trait_exerci::mod_trait::TraitCanal;
use mod_trait_exerci::mod_trait::TupleType;

// clear && cargo run --example box_dynamic_hello
fn main() {
    let instance: StructType = Default::default();
    let instance_box_trait: Box<dyn TraitCanal> = Box::new(instance);
    assert_eq!((0), mod_dynamic_fn::get_dynamic_box_ref(&instance_box_trait));
    assert_eq!((0), mod_dynamic_fn::get_dynamic_box(instance_box_trait));

    let instance: StructType = Default::default();
    let instance_box_trait: Box<&dyn TraitCanal> = Box::new(&instance);
    assert_eq!((0), mod_dynamic_fn::get_dynamic_box_and_trait_ref(&instance_box_trait));
    assert_eq!((0), mod_dynamic_fn::get_dynamic_box_trait_ref(instance_box_trait));

    let instance: TupleType = Default::default();
    let instance_box_trait: Box<dyn TraitCanal> = Box::new(instance);
    assert_eq!((0), mod_dynamic_fn::get_dynamic_box_ref(&instance_box_trait));
    assert_eq!((0), mod_dynamic_fn::get_dynamic_box(instance_box_trait));

    let instance: TupleType = Default::default();
    let instance_box_trait: Box<&dyn TraitCanal> = Box::new(&instance);
    assert_eq!((0), mod_dynamic_fn::get_dynamic_box_and_trait_ref(&instance_box_trait));
    assert_eq!((0), mod_dynamic_fn::get_dynamic_box_trait_ref(instance_box_trait));
}
