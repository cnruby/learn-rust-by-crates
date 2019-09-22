use trait_exerci::CanalTrait;

#[test]
fn it_works() {
    let mine = trait_exerci::StructType { data: 0 };
    assert_eq!(0, mine.foo());
}
