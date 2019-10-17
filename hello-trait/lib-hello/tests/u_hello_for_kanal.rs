use trait_exerci::StructType;
use trait_exerci::TraitKanal;

#[test]
fn it_works_with_set() {
    let mut instance = StructType::new(0);
    instance.set_data(&30);
    assert_eq!(StructType::new(30), instance);
}

#[test]
fn it_works_with_default() {
    let mut instance :StructType = Default::default();
    instance.set_data(&30);
    assert_eq!(StructType::new(30), instance);
}
