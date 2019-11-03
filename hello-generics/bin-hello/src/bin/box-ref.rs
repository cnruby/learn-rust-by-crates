#[derive(Debug, Default, PartialEq, Copy, Clone)]
struct StructType {
    data: u32,
}

trait TraitCanal {
    fn get(&self) -> u32;
}

impl TraitCanal for StructType {
    fn get(&self) -> u32 {
        println!("impl TraitCanal for StructType");
        self.data
    }
}

fn get_static_ref<Type>(typ: &Type) -> u32
where
    Type: TraitCanal,
{
    println!("get_static_ref()");
    typ.get()
}

fn get_static_box<Type>(typ: Box<Type>) -> u32
where
    Type: TraitCanal,
{
    println!("get_static_box()");
    typ.get()
}

// clear && cargo run --bin box-ref
fn main() {
    let instance: StructType = Default::default();
    assert_eq!(0, get_static_ref(&instance));
    println!("");
    let instance_box_type: Box<StructType> = Box::new(instance);
    assert_eq!(0, get_static_box(instance_box_type));
}
