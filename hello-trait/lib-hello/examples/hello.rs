use trait_exerci::StructType;
use trait_exerci::TraitCanal;
use trait_exerci::TraitKanal;

// cargo run --example hello
fn main() {
    let instance = StructType::new(100);
    assert_eq!(StructType::new(100), instance);
    println!("{}", instance.get_data());

    let instance = Box::new(StructType::new(101));
    assert_eq!(Box::new(StructType::new(101)), instance);
    println!("{}", instance.get_data());

    let mut instance = StructType::new(102);
    instance.set_data(&103);
    assert_eq!(StructType::new(103), instance);
    println!("{}", instance.get_data_for_all());

    let mut instance = Box::new(StructType::new(104));
    instance.set_data_for_all(&105);
    assert_eq!(Box::new(StructType::new(105)), instance);
    println!("{}", instance.get_data());

    let instance :StructType = Default::default();
    assert_eq!(StructType::new(0), instance);
    println!("{}", instance.get_data());
}
