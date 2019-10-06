// For `impl TraitKanal for StructType`
use trait_exerci::TraitKanal;

// For `impl StructType for StructType`
// AND For `impl StructType`
use trait_exerci::StructType;

#[test]
fn it_works_with_set() {
    let mut instance = StructType::new(0);
    instance.set_data(&31);
    assert_eq!(31, instance.data);
}
