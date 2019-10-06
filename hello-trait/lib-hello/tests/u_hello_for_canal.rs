// For `impl TraitCanal for StructType`
use trait_exerci::TraitCanal;

// For `impl StructType for StructType`
// AND For `impl StructType`
use trait_exerci::StructType;

#[test]
fn it_works_with_get() {
    let instance = StructType { data: 20 };
    assert_eq!(20, instance.get_data());
}
