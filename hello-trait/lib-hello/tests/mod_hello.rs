#[cfg(test)]
mod tests {
    use trait_exerci::TraitCanal;

    #[test]
    fn it_works() {
        let instance = Box::new(trait_exerci::StructType { data: 20 });
        assert_eq!(20, instance.get_data());
    }

    #[test]
    fn it_works_with_new() {
        let instance = Box::new(trait_exerci::StructType::new(21));
        assert_eq!(21, instance.get_data());
    }
}
