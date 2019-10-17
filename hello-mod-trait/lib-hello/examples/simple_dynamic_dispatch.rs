// File: examples/simple_dynamic_dispatch.rs
trait Trait {
    fn static_dispatch(&self) -> (u8);
}

#[derive(Default)]
struct Struct {data: (u8)}

impl Trait for Struct {
    fn static_dispatch(&self) -> (u8) {
        (self.data)
    }
}

// clear && cargo run --example simple_dynamic_dispatch
fn main () {
    let instance_struct: Struct = Default::default();
    assert_eq!((0u8), instance_struct.static_dispatch());

    let instance_struct: Struct = Default::default();
    let box_struct: Box<dyn Trait> = Box::new(instance_struct);
    assert_eq!((0), box_struct.static_dispatch());

    let instance_struct: Struct = Default::default();
    let mut v: Vec<&dyn Trait> = Vec::new();
    v.push(&instance_struct);
    for instance_struct in v.iter() {
        assert_eq!((0), instance_struct.static_dispatch());
    }

    let instance_struct: Struct = Default::default();
    let mut v: Vec<Box<dyn Trait>> = Vec::new();
    v.push(Box::new(instance_struct));
    for instance_struct in v.iter() {
        assert_eq!((0), instance_struct.static_dispatch());
    }    
}