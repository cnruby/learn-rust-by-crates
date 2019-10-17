#![allow(unused_variables)]
struct Struct<T> (T);

trait Trait<U> {
    fn get(&self) -> &U;
}

impl<S> Trait<S> for Struct<S> {
    fn get(&self) -> &S {
        &self.0
    }
}

// clear && cargo run --example generics_trait_hello
fn main() {
    let instance = Struct(0u8);
    instance.get();
    let instance = Struct(0u32);
    instance.get();

    let instance = Struct('0');
    instance.get();
    let instance = Struct("0");
    instance.get();
}