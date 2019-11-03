use super::mod_generics::Trait;

pub fn get_static_all<Type>(typ: &Type) -> u32
    where Type: Trait {
    typ.get()
}