// File: src/mod_dynamic_fn.rs
use super::mod_trait::TraitCanal;

pub fn get_dynamic_trait_ref(canal: &dyn TraitCanal) -> (u32) {
    (canal.get_tuple())
}
