// For `impl TraitCanal for StructType`
use trait_exerci::TraitCanal;
// For `impl TraitKanal for StructType`
use trait_exerci::TraitKanal;

// For `impl AnyTrait for StructType` AND For `impl StructType`
use trait_exerci::StructType;

#[test]
fn it_works_with_both_traits() {
    let mut instance = StructType::new(0);
    instance.set_data(&41);
    assert_eq!(41, instance.get_data());
    assert_eq!(StructType::new(41), instance);
}
