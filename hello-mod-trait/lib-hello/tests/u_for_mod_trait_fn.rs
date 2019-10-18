// File: tests/u_for_mod_trait_fn.rs
// clear && cargo test
// clear && cargo test --package mod_trait_exerci
// $ clear && cargo test --test u_for_mod_trait_fn
#[cfg(test)]
mod tests {
    use mod_trait_exerci::mod_static_fn;
    use mod_trait_exerci::mod_dynamic_fn;
    use mod_trait_exerci::mod_trait;
    use mod_trait::StructType;
    use mod_trait::TupleType;

    #[test]
    fn it_works_with_fn_struct_default() {
        let instance: StructType = Default::default();
        let data = mod_static_fn::get_static_type_ref(&instance);
        assert_eq!(0, data);
        assert_eq!((0), data);
        let data = mod_dynamic_fn::get_dynamic_trait_ref(&instance);
        assert_eq!(0, data);
        assert_eq!((0), data);
    }

    #[test]
    fn it_works_with_fn_tuple_default() {
        let instance: TupleType = Default::default();
        let data = mod_static_fn::get_static_type_ref(&instance);
        assert_eq!(0, data);
        assert_eq!((0), data);
        let data = mod_dynamic_fn::get_dynamic_trait_ref(&instance);
        assert_eq!(0, data);
        assert_eq!((0), data);
    }
}