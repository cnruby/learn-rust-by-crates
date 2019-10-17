// File: src/mod_dynamic_fn.rs
use super::mod_trait::TraitCanal;

pub fn get_dynamic_trait_ref(canal: &dyn TraitCanal) -> u32 {
    canal.get()
}

pub fn get_dynamic_box(canal: Box<dyn TraitCanal>) -> u32 {
    canal.get()
}

pub fn get_dynamic_box_trait_ref(canal: Box<&dyn TraitCanal>) -> u32 {
    canal.get()
}

pub fn get_dynamic_box_ref(canal: &Box<dyn TraitCanal>) -> u32 {
    canal.get()
}

pub fn get_dynamic_box_and_trait_ref(canal: &Box<&dyn TraitCanal>) -> u32 {
    canal.get()
}