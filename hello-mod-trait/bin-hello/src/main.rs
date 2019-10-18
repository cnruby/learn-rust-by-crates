fn main() {
    use mod_trait_exerci::mod_trait;
    let instance = mod_trait::StructType { data: 10 };
    println!("{}", instance.get());
    let instance = Box::new(mod_trait::StructType { data: 10 });
    println!("{}", instance.get());

    use mod_trait_exerci::mod_bare;
    let instance = mod_bare::StructType { data: 10 };
    println!("{}", instance.get());
    let instance = Box::new(mod_bare::StructType { data: 20 });
    println!("{}", instance.get());

    use mod_trait::TraitCanal as ModTraitCanal;
    let instance = mod_trait::StructType { data: 100 };
    println!("{}", instance.get());
    let instance = Box::new(mod_trait::StructType { data: 200 });
    println!("{}", instance.get());
}
