// FROM ROOT bin-local-hello
// clear && cargo test
// FROM ROOT ../bin-local-hello
// clear && cargo test --package bin-local-hello
#[cfg(test)]
mod tests {
    use mod_trait_exerci::mod_dynamic_fn;
    use mod_trait_exerci::mod_trait;
    use mod_trait::StructType;
    use mod_trait::TraitCanal;
    use mod_trait::TupleType;

    #[test]
    fn struct_dynamic_box() {
        let instance: StructType = Default::default();
        let instance_box_trait: Box<dyn TraitCanal> = Box::new(instance);
        assert_eq!(0, mod_dynamic_fn::get_dynamic_box_ref(&instance_box_trait));
        assert_eq!(0, mod_dynamic_fn::get_dynamic_box(instance_box_trait));
    }

    #[test]
    fn struct_dynamic_box_and_trait() {
        let instance: StructType = Default::default();
        let instance_box_trait: Box<&dyn TraitCanal> = Box::new(&instance);
        assert_eq!(0, mod_dynamic_fn::get_dynamic_box_and_trait_ref(&instance_box_trait));
        assert_eq!(0, mod_dynamic_fn::get_dynamic_box_trait_ref(instance_box_trait));
    }

    #[test]
    fn tulpe_dynamic_box() {
        let instance: TupleType = Default::default();
        let instance_box_trait: Box<dyn TraitCanal> = Box::new(instance);
        assert_eq!(0, mod_dynamic_fn::get_dynamic_box_ref(&instance_box_trait));
        assert_eq!(0, mod_dynamic_fn::get_dynamic_box(instance_box_trait));
    }

    #[test]
    fn tulpe_dynamic_box_trait() {
        let instance: TupleType = Default::default();
        let instance_box_trait: Box<&dyn TraitCanal> = Box::new(&instance);
        assert_eq!(0, mod_dynamic_fn::get_dynamic_box_and_trait_ref(&instance_box_trait));
        assert_eq!(0, mod_dynamic_fn::get_dynamic_box_trait_ref(instance_box_trait));
    }
}