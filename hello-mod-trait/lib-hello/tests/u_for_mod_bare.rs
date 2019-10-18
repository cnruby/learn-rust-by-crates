// File: tests/u_for_mod_bare.rs
// clear && cargo test
// clear && cargo test --package mod_trait_exerci
// $ clear && cargo test --test u_for_mod_bare
#[cfg(test)]
mod tests {
    use mod_trait_exerci::mod_bare;
    use mod_bare::StructType;
    use mod_bare::TupleType;

    #[test]
    fn it_works_with_struct_default() {
        let instance: StructType = Default::default();
        assert_eq!(0, instance.get());
        assert_eq!((0), instance.get());
    }

    #[test]
    fn it_works_with_struct_struct() {
        let instance = StructType{data:(0)};
        assert_eq!(0, instance.get());
        assert_eq!((0), instance.get());
    }

    #[test]
    fn it_works_with_struct() {
        let instance_default: StructType = Default::default();
        let instance_struct = StructType{data:(0)};
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
        let instance = TupleType(0);
        assert_eq!(0, instance.get());
        assert_eq!((0), instance.get());
    }

    #[test]
    fn it_works_with_tuple() {
        let instance_default: TupleType = Default::default();
        let instance_tuple = TupleType(0);
        assert_eq!(instance_default, instance_tuple);
    }
}