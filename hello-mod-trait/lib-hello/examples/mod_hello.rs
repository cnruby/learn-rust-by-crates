// $ cargo run --example mod_hello
use mod_trait_exerci::mod_trait;
use mod_trait_exerci::mod_trait::CanalTrait;

fn main() {
    let mine = mod_trait::StructType { data: 101 };
    println!("{}", mine.foo());

    let mine = Box::new(mod_trait::StructType { data: 102 });
    println!("{}", mine.foo());
}