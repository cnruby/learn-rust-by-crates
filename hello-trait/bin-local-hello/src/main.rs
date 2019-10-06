// $ cargo run main
use trait_exerci::TraitCanal;

fn main() {
    let instance = trait_exerci::StructType { data: 10 };
    println!("{}", instance.get_data());

    let instance = Box::new(trait_exerci::StructType { data: 10 });
    println!("{}", instance.get_data());
}