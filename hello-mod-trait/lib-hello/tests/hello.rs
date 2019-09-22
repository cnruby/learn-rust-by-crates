use mod_trait_exerci::mod_trait;
use mod_trait_exerci::mod_trait::CanalTrait;

use mod_trait_exerci::mod_bare;

#[test]
fn it_works_with_mod() {
    let mine = mod_trait::StructType { data: 0 };
    assert_eq!(0, mine.foo());
}

#[test]
fn it_works() {
    let mine = mod_bare::StructType { data: 30 };
    assert_eq!(30, mine.foo());
}
