// For `impl StructType for StructType`
// AND For `impl StructType`
use trait_exerci::StructType;

#[test]
fn it_works_with_data_and_get() {
    // pub data: u32,
    let instance = StructType { data: 10 };
    assert_eq!(10, instance.get_data_for_all());
}

#[test]
fn it_works_with_new_and_get() {
    // new() + get()
    let instance = StructType::new(11);
    assert_eq!(11, instance.get_data_for_all());
}

#[test]
fn it_works_with_set() {
    let mut instance = StructType::new(0);
    instance.set_data_for_all(&12);
    assert_eq!(12, instance.get_data_for_all());
}
