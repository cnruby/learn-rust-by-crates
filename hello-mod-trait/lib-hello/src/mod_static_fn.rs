// File: src/mod_static_fn.rs
use super::mod_trait::TraitCanal;

pub fn get_static_type_ref<Type: TraitCanal>(typ: &Type) -> u32 {
    typ.get()
}

pub fn get_static_box_ref<Type: TraitCanal>(typ: &Box<Type>) -> u32 {
    typ.get()
}

pub fn get_static_box<Type: TraitCanal>(typ: Box<Type>) -> u32 {
    typ.get()
}

pub fn get_static_box_type_ref<Type: TraitCanal + ?Sized>(typ: Box<&Type>) -> u32 {
    typ.get()
}

pub fn get_static_box_and_type_ref<Type: TraitCanal + ?Sized>(typ: &Box<&Type>) -> u32 {
    typ.get()
}

