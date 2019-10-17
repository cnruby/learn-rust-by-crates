use trait_exerci::StructType;
use trait_exerci::TraitCanal;
use trait_exerci::TraitKanal;

#[test]
fn it_works_with_both_traits_and_new() {
    let mut instance = StructType::new(40);
    instance.set_data(&41);
    assert_eq!(41, instance.get_data());
    assert_eq!(StructType::new(41), instance);
}

#[test]
fn it_works_with_both_traits_and_default() {
    let mut instance :StructType = Default::default();
    instance.set_data(&41);
    assert_eq!(41, instance.get_data());
    assert_eq!(StructType::new(41), instance);
}
