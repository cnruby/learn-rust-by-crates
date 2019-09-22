// $ cargo run local_main
use local_trait_exerci::CanalTrait;

fn main() {
    let mine = local_trait_exerci::StructType { data: 10 };
    println!("{}", mine.foo());

    let mine = Box::new(local_trait_exerci::StructType { data: 0 });
    println!("{}", mine.foo());
}