#[cfg(test)]
mod tests {
    use mod_trait_exerci::mod_static_fn;
    use mod_trait_exerci::mod_dynamic_fn;
    use mod_trait_exerci::mod_where_fn;
    use mod_trait_exerci::mod_trait;

    #[test]
    fn struct_type_ref() {
        let instance: mod_trait::StructType = Default::default();
        //assert_eq!(0, instance.get());
        assert_eq!(0, mod_static_fn::get_static_type_ref(&instance));
        assert_eq!(0, mod_dynamic_fn::get_dynamic_trait_ref(&instance));
    }

    #[test]
    fn tuple_type_ref() {
        let instance: mod_trait::TupleType = Default::default();
        //assert_eq!(0, instance.get());
        assert_eq!(0, mod_static_fn::get_static_type_ref(&instance));
        assert_eq!(0, mod_dynamic_fn::get_dynamic_trait_ref(&instance));
    }

    #[test]
    fn where_type_ref() {
        let instance: mod_trait::TupleType = Default::default();
        assert_eq!(0, mod_where_fn::get_static_type_ref_with_where(&instance));
    }
}