#[cfg(test)]
mod tests {
    use trait_exerci::CanalTrait;

    #[test]
    fn it_works() {
        let mine = Box::new(trait_exerci::StructType { data: 10 });
        assert_eq!(10, mine.foo());
    }
}
