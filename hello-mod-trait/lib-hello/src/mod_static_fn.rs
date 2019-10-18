// File: src/mod_static_fn.rs
use super::mod_trait::TraitCanal;
use std::fmt::Debug;
use std::cmp::PartialEq;

pub fn get_static_type_ref<Type: TraitCanal>(typ: &Type) -> (u32) {
    (typ.get_tuple())
}

pub fn print_static_all_daten<Type: TraitCanal+Debug+PartialEq>(typs: &[Type]) {
    for typ in typs {
        let data = typ.get_tuple();
        println!("{:?}", typ);          // FOR Debug
        println!("{:?}", data);         // FOR Debug
        assert_eq!(*typ, typ.get_object());   // FOR PartialEq
    }
}