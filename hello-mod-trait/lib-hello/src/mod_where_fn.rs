// File: src/mod_where_fn.rs
use super::mod_trait::TraitCanal;

pub fn get_static_type_ref<Type: TraitCanal>(typ: &Type) -> (u32) {
    typ.get_tuple()
}

pub fn get_static_type_ref_with_where<Type>(typ: &Type) -> (u32)
    where Type: TraitCanal {
    typ.get_tuple()
}
