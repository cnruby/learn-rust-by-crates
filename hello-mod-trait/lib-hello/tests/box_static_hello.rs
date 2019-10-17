// FROM ROOT bin-local-hello
// clear && cargo test
// FROM ROOT ../bin-local-hello
// clear && cargo test --package bin-local-hello
#[cfg(test)]
mod tests {
    use mod_trait_exerci::mod_static_fn;
    use mod_trait_exerci::mod_trait;
    use mod_trait::StructType;
    use mod_trait::TraitCanal;
    use mod_trait::TupleType;

    #[test]
    fn struct_static_box() {
        let instance: StructType = Default::default();
        let instance_box_type: Box<StructType> = Box::new(instance);
        assert_eq!(0, mod_static_fn::get_static_type_ref(&instance));
        //assert_eq!(0, mod_static_fn::get_static_type_ref(&instance_box_type));
        //assert_eq!(0, mod_static_fn::get_static_box_ref(&instance));
        assert_eq!(0, mod_static_fn::get_static_box_ref(&instance_box_type));
        assert_eq!(0, mod_static_fn::get_static_box(instance_box_type));
    }

    #[test]
    fn struct_static_box_ref_and_type_ref() {
        let instance: StructType = Default::default();
        let instance_box_type: Box<&dyn TraitCanal> = Box::new(&instance);
        assert_eq!(0, mod_static_fn::get_static_box_and_type_ref(&instance_box_type));
        assert_eq!(0, mod_static_fn::get_static_box_type_ref(instance_box_type));
    }

    #[test]
    fn tuple_static_box() {
        let instance: TupleType = Default::default();
        let instance_box_type: Box<TupleType> = Box::new(instance);
        assert_eq!(0, mod_static_fn::get_static_box_ref(&instance_box_type));
        assert_eq!(0, mod_static_fn::get_static_box(instance_box_type));
    }

    #[test]
    fn tuple_static_box_type() {
        let instance: TupleType = Default::default();
        let instance_box_type: Box<&dyn TraitCanal> = Box::new(&instance);
        assert_eq!(0, mod_static_fn::get_static_box_and_type_ref(&instance_box_type));
        assert_eq!(0, mod_static_fn::get_static_box_type_ref(instance_box_type));
    }
}
