fn main() {
    use mod_trait_exerci::mod_trait;
    let instance = mod_trait::StructType::new(10);
    println!("{}", instance.get_tuple());
    let instance = Box::new(mod_trait::StructType::new(10));
    println!("{}", instance.get_tuple());

    use mod_trait_exerci::mod_bare;
    let instance = mod_bare::StructType { data: 10 };
    println!("{}", instance.get_tuple());
    let instance = Box::new(mod_bare::StructType { data: 10 });
    println!("{}", instance.get_tuple());

    use mod_trait::TraitCanal as ModTraitCanal;
    let instance = mod_trait::StructType::new(10);
    println!("{}", instance.get_tuple());
    let instance = Box::new(mod_trait::StructType::new(10));
    println!("{}", instance.get_tuple());
}
