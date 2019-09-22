#[cfg(test)]
mod tests {
    use mod_trait_exerci::mod_trait;
    use mod_trait_exerci::mod_trait::CanalTrait;

    #[test]
    fn it_works() {
        let mine = Box::new(mod_trait::StructType { data: 10 });
        assert_eq!(10, mine.foo());
    }
}
