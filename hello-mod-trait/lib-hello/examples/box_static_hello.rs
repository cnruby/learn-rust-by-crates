use mod_trait_exerci::mod_static_fn;
use mod_trait_exerci::mod_trait::StructType;
use mod_trait_exerci::mod_trait::TraitCanal;
use mod_trait_exerci::mod_trait::TupleType;

// clear && cargo run --example box_static_hello
fn main() {
    let instance: StructType = Default::default();
    let instance_box_type: Box<StructType> = Box::new(instance);
    assert_eq!((0), mod_static_fn::get_static_box_ref(&instance_box_type));
    assert_eq!((0), mod_static_fn::get_static_box(instance_box_type));

    let instance: StructType = Default::default();
    let instance_box_type: Box<&dyn TraitCanal> = Box::new(&instance);
    assert_eq!((0), mod_static_fn::get_static_box_and_type_ref(&instance_box_type));
    assert_eq!((0), mod_static_fn::get_static_box_type_ref(instance_box_type));

    let instance: TupleType = Default::default();
    let instance_box_type: Box<TupleType> = Box::new(instance);
    assert_eq!((0), mod_static_fn::get_static_box_ref(&instance_box_type));
    assert_eq!((0), mod_static_fn::get_static_box(instance_box_type));

    let instance: TupleType = Default::default();
    let instance_box_type: Box<&dyn TraitCanal> = Box::new(&instance);
    assert_eq!((0), mod_static_fn::get_static_box_and_type_ref(&instance_box_type));
    assert_eq!((0), mod_static_fn::get_static_box_type_ref(instance_box_type));
}
