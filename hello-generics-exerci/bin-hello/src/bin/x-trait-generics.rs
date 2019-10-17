#[derive(Debug, Default, PartialEq, Copy, Clone)]
struct StructType();

trait TraitCanal<T> {
    fn get(&self) -> ();
}

impl<T, U> TraitCanal<T> for U {
    fn get(&self) -> () {
        println!("impl TraitCanal<T> for AllType");
        ()
    }
}

/*
impl TraitCanal for &StructType {
    fn get(&self) -> u32 {
        println!("impl TraitCanal for &StructType");
        self.data
    }
}
*/

/*
impl TraitCanal for Box<StructType> {
    fn get(&self) -> u32 {
        println!("impl TraitCanal for Box<StructType>");
        self.data
    }
}
*/

fn get_static<Type, U>(typ: Type) -> ()
where
    Type: TraitCanal<StructType>,
    Type: std::ops::Deref<Target = U>,
    U: TraitCanal<StructType> + ?Sized,
{
    println!("get_static()");
    typ.get()
}

// clear && cargo run --bin trait-generics
fn main() {
    let instance: StructType = Default::default();
    assert_eq!((), get_static(&instance));
    println!("");
    let instance_box_type: Box<StructType> = Box::new(instance);
    assert_eq!((), get_static(instance_box_type));
}
