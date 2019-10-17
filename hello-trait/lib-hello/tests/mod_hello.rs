#[cfg(test)]
mod tests {
    use trait_exerci::StructType;
    use trait_exerci::TraitCanal;

    #[test]
    fn it_works_with_new() {
        let instance = StructType::new(50);
        assert_eq!(StructType::new(50), instance);
    }

    #[test]
    fn it_works_with_get() {
        let instance = StructType::new(51);
        assert_eq!(51, instance.get_data());
    }

    #[test]
    fn it_works_with_new_and_box() {
        let instance = Box::new(StructType::new(52));
        assert_eq!(Box::new(StructType::new(52)), instance);
    }

    #[test]
    fn it_works_with_get_and_box() {
        let instance = Box::new(StructType::new(53));
        assert_eq!(53, instance.get_data());
    }

    use trait_exerci::TraitKanal;
    #[test]
    fn it_works_with_set() {
        let mut instance = StructType::new(54);
        instance.set_data(&55);
        assert_eq!(StructType::new(55), instance);
    }

    #[test]
    fn it_works_with_set_and_box() {
        let mut instance = Box::new(StructType::new(56));
        instance.set_data(&57);
        assert_eq!(Box::new(StructType::new(57)), instance);
    }
}
