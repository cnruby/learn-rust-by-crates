// File: examples/simple_static_dispatch.rs
#[derive(Default)]
struct NormalStruct {
    data: (u8)
}

impl NormalStruct {
    fn static_dispatch(&self) -> (u8) { (self.data) }
}

// clear && cargo run --example simple_static_dispatch
fn main() {
    let instance_struct :NormalStruct = Default::default();
    assert_eq!((0u8), instance_struct.static_dispatch());
}