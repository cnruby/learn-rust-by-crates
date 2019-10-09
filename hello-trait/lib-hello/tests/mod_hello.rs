#[cfg(test)]
mod tests {
    use trait_exerci::StructType;
    use trait_exerci::TraitCanal;

    #[test]
    fn it_works_with_new() {
        let instance = StructType::new(21);
        assert_eq!(StructType::new(21), instance);
    }

    #[test]
    fn it_works_with_get() {
        let instance = StructType::new(21);
        assert_eq!(21, instance.get_data());
    }

    #[test]
    fn it_works_with_new_and_box() {
        let instance = Box::new(StructType::new(22));
        assert_eq!(Box::new(StructType::new(22)), instance);
    }

    #[test]
    fn it_works_with_get_and_box() {
        let instance = Box::new(StructType::new(22));
        assert_eq!(22, instance.get_data());
    }

    use trait_exerci::TraitKanal;
    #[test]
    fn it_works_with_set() {
        let mut instance = StructType::new(23);
        instance.set_data(&24);
        assert_eq!(StructType::new(24), instance);
    }

    #[test]
    fn it_works_with_set_and_box() {
        let mut instance = Box::new(StructType::new(25));
        instance.set_data(&26);
        assert_eq!(Box::new(StructType::new(26)), instance);
    }
}
