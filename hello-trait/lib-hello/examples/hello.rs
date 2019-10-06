use trait_exerci::TraitCanal;

fn main() {
    let instance = trait_exerci::StructType { data: 100 };
    println!("{}", instance.get_data());

    let instance = Box::new(trait_exerci::StructType { data: 101 });
    println!("{}", instance.get_data());

    let instance = trait_exerci::StructType::new(102);
    println!("{}", instance.get_data());

    let instance = Box::new(trait_exerci::StructType::new(103));
    println!("{}", instance.get_data());
}
