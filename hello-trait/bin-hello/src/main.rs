// $ cargo run main
use trait_exerci::CanalTrait;

fn main() {
    let mine = trait_exerci::StructType { data: 10 };
    println!("{}", mine.foo());

    let mine = Box::new(trait_exerci::StructType { data: 0 });
    println!("{}", mine.foo());
}