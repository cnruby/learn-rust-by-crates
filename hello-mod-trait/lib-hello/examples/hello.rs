// $ cargo run --example hello
use mod_trait_exerci::mod_bare;

fn main() {
    let mine = mod_bare::StructType { data: 103 };
    println!("{}", mine.foo());

    let mine = Box::new(mod_bare::StructType { data: 104 });
    println!("{}", mine.foo());
}