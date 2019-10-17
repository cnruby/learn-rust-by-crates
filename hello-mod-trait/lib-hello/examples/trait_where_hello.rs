// File: examples/trait_where_hello.rs
use mod_trait_exerci::mod_trait;
use mod_trait::StructType;
use mod_trait::TupleType;
use mod_trait_exerci::mod_where_fn;

fn static_struct_ref_with_where(instance: &StructType) {
    let data = mod_where_fn::get_static_type_ref_with_where(instance);
    assert_eq!(0, data);
    assert_eq!((0), data);
}

fn static_tuple_ref_with_where(instance: &TupleType) {
    let data = mod_where_fn::get_static_type_ref_with_where(instance);
    assert_eq!(0, data);
    assert_eq!((0), data);
}

// clear && cargo run --example trait_where_hello
fn main() {
    let instance: StructType = Default::default();
    static_struct_ref_with_where(&instance);

    let instance: TupleType = Default::default();
    static_tuple_ref_with_where(&instance);
}