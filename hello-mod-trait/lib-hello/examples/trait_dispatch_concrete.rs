// File: examples/trait_dispatch_concrete.rs
// 1. Define and Implement struct, trait and impl..for
struct NormalStruct {
    data: (u8)
}

trait Trait {
    fn _fn(&self) -> (u8);
}

impl Trait for NormalStruct {
    fn _fn(&self) -> (u8) { (self.data) }
}

struct TupleStruct(u8);

impl Trait for TupleStruct {
    fn _fn(&self) -> (u8) { (self.0) }
}

// 2. Implement static and dynamic dispatch
fn static_dispatch<TraitObject: Trait>(r#type: &TraitObject) {
    r#type._fn(); 
}

fn dynamic_dispatch(r#trait: &dyn Trait) {
    r#trait._fn();
}

// 3. Use these dispatch functions
// clear && cargo run --example trait_dispatch_concrete
fn main() {
    let instance_struct = NormalStruct{data: 0};
    assert_eq!((), static_dispatch(&instance_struct));
    assert_eq!((), dynamic_dispatch(&instance_struct));

    let instance_tuple = TupleStruct(0);
    assert_eq!((), static_dispatch(&instance_tuple));
    assert_eq!((), dynamic_dispatch(&instance_tuple));
}