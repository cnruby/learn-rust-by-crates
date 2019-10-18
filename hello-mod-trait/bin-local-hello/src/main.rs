use mod_trait_exerci::mod_bare;
use mod_trait_exerci::mod_trait;
use mod_trait_exerci::mod_trait::TraitCanal;

// cargo run
fn main() {
    let instance :mod_bare::StructType = Default::default();
    println!("{}", instance.get_tuple());

    let instance: Box<mod_bare::StructType> = Box::new(Default::default());
    println!("{}", instance.get_tuple());

    let instance :mod_trait::StructType = Default::default();
    println!("{}", instance.get_tuple());

    let instance: Box<mod_trait::StructType> = Box::new(Default::default());
    println!("{}", instance.get_tuple());

    let instance :mod_bare::TupleType = Default::default();
    println!("{}", instance.get_tuple());

    let instance: Box<mod_bare::TupleType> = Box::new(Default::default());
    println!("{}", instance.get_tuple());

    let instance :mod_trait::TupleType = Default::default();
    println!("{}", instance.get_tuple());

    let instance: Box<mod_trait::TupleType> = Box::new(Default::default());
    println!("{}", instance.get_tuple());

}

