// File: src/mod_dynamic_fn.rs
use super::mod_trait::TraitCanal;

pub fn get_dynamic_box(canal: Box<dyn TraitCanal>) -> (u32) {
    canal.get_tuple()
}

pub fn get_dynamic_box_trait_ref(canal: Box<&dyn TraitCanal>) -> (u32) {
    canal.get_tuple()
}

pub fn get_dynamic_box_ref(canal: &Box<dyn TraitCanal>) -> (u32) {
    canal.get_tuple()
}

pub fn get_dynamic_box_and_trait_ref(canal: &Box<&dyn TraitCanal>) -> (u32) {
    canal.get_tuple()
}