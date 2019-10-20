// File: src/mod_static_fn.rs
use super::mod_trait::TraitCanal;
use std::fmt::Debug;
use std::cmp::PartialEq;

//get_static_type_ref<T>
//get_static_trait_ref::<StructType>
//get_static_trait_ref::<TupleType>

// get_static_type_ref<Type: TraitCanal>(typ: &Type) -> (u32)
//get_static_trait_ref(typ: &StructType) -> (u32)
//get_static_trait_ref(typ: &TupleType) -> (u32)

// static: Generics type parameters
// dynamic: trait objects

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