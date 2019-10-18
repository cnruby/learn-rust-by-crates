// File: tests/u_for_mod_trait_instance.rs
// clear && cargo test
// clear && cargo test --package mod_trait_exerci
// $ clear && cargo test --test u_for_mod_trait_instance
#[cfg(test)]
mod tests {
    use mod_trait_exerci::mod_trait;
    use mod_trait::TraitCanal;
    use mod_trait::StructType;
    use mod_trait::TupleType;

    #[test]
    fn it_works_with_struct_default() {
        let instance: StructType = Default::default();
        assert_eq!(0, instance.get());
        assert_eq!((0), instance.get());
    }

    #[test]
    fn it_works_with_struct_new() {
        let instance = StructType::new(0);
        assert_eq!(0, instance.get());
        assert_eq!((0), instance.get());
    }

    #[test]
    fn it_works_with_struct() {
        let instance_default: StructType = Default::default();
        let instance_struct = StructType::new(0);
        assert_eq!(instance_default, instance_struct);
    }

    #[test]
    fn it_works_with_tuple_default() {
        let instance: TupleType = Default::default();
        assert_eq!(0, instance.get());
        assert_eq!((0), instance.get());
    }

    #[test]
    fn it_works_with_tuple_struct() {
        let instance = TupleType::new(0);
        assert_eq!(0, instance.get());
        assert_eq!((0), instance.get());
    }

    #[test]
    fn it_works_with_tuple() {
        let instance_default: TupleType = Default::default();
        let instance_tuple = TupleType::new(0);
        assert_eq!(instance_default, instance_tuple);
    }
}