use trait_exerci::StructType;

#[test]
fn it_works_with_new() {
    let instance = StructType::new(10);
    assert_eq!(StructType::new(10), instance);
}

#[test]
fn it_works_with_default() {
    let instance :StructType = Default::default();
    assert_eq!(StructType::new(0), instance);
}

#[test]
fn it_works_with_get() {
    let instance = StructType::new(11);
    assert_eq!(11, instance.get_data_for_all());
    assert_eq!(StructType::new(11), instance);
}

#[test]
fn it_works_with_set() {
    let mut instance = StructType::new(0);
    instance.set_data_for_all(&12);
    assert_eq!(StructType::new(12), instance);
}
