#![allow(unused_variables)]
struct Struct<T> (T);

impl<S> Struct<S> {
    fn get(&self) -> &S {
        &self.0
    }
}
// clear && cargo run --example generics_impl_hello
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