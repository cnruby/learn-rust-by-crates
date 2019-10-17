use trait_exerci::StructType;
use trait_exerci::TraitCanal;

#[test]
fn it_works_with_get() {
    let instance = StructType::new(20);
    assert_eq!(20, instance.get_data());
    assert_eq!(StructType::new(20), instance);
}

#[test]
fn it_works_with_default() {
    let instance :StructType = Default::default();
    assert_eq!(0, instance.get_data());
    assert_eq!(StructType::new(0), instance);
}
