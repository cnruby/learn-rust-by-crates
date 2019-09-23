use mod_trait_exerci::mod_bare;
use mod_trait_exerci::mod_trait;
use mod_trait_exerci::mod_trait::CanalTrait;

fn main() {
    let mine = mod_trait::StructType { data: 100 };
    println!("{}", mine.foo());

    let mine = Box::new(mod_trait::StructType { data: 200 });
    println!("{}", mine.foo());

    let mine = mod_bare::StructType { data: 10 };
    println!("{}", mine.foo());

    let mine = Box::new(mod_bare::StructType { data: 20 });
    println!("{}", mine.foo());
}

