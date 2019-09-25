use trait_exercism;

fn main() {
let mine = trait_exercism::MyType { data: 0 };
println!("{}", mine.foo());

//let mine = Box::new(MyType { data: 0 });
//println!("{}", mine.foo());
}