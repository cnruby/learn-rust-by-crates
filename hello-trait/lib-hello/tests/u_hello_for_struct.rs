// Only For `impl StructType`
use trait_exerci::StructType;

#[test]
fn it_works_with_new_and_get() {
    let instance = StructType::new(10);
    assert_eq!(10, instance.get_data_for_all());
    assert_eq!(StructType::new(10), instance);
}

#[test]
fn it_works_with_set() {
    let mut instance = StructType::new(0);
    instance.set_data_for_all(&12);
    assert_eq!(StructType::new(12), instance);
}
