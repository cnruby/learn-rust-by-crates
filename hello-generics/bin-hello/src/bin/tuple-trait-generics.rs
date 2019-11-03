#[derive(Debug, Default, PartialEq, Copy, Clone)]
struct StructType();

trait TraitCanal<T> {
    fn get(&self) -> ();
}

impl<Struct, AllStruct> TraitCanal<Struct> for AllStruct {
    fn get(&self) -> () {
        println!("impl TraitCanal<T> for AllType");
        ()
    }
}

fn get_static<Type>(typ: Type) -> ()
where
    Type: TraitCanal<StructType>,
{
    println!("get_static()");
    typ.get()
}

// clear && cargo run --bin tuple-trait-generics
fn main() {
    let instance: StructType = Default::default();
    assert_eq!((), get_static(&instance));
    println!("");
    let instance_box_type: Box<StructType> = Box::new(instance);
    assert_eq!((), get_static(&instance_box_type));
    println!("");
    let instance_box_type: Box<&StructType> = Box::new(&instance);
    assert_eq!((), get_static(&instance_box_type));
    println!("");
    let instance_box_type: Box<StructType> = Box::new(instance);
    assert_eq!((), get_static(instance_box_type));
    println!("");
    let instance: StructType = Default::default();
    let instance_box_type: Box<&StructType> = Box::new(&instance);
    assert_eq!((), get_static(instance_box_type));
}
