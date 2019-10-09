use trait_exerci::StructType;
use trait_exerci::TraitCanal;
use trait_exerci::TraitKanal;

fn main() {
    let instance = StructType::new(100);
    println!("{}", instance.get_data());

    let instance = Box::new(StructType::new(101));
    println!("{}", instance.get_data());

    let mut instance = StructType::new(102);
    instance.set_data(&103);
    println!("{}", instance.get_data());

    let mut instance = Box::new(StructType::new(104));
    instance.set_data(&105);
    println!("{}", instance.get_data());
}
