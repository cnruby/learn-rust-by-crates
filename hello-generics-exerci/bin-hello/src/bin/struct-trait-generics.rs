#[derive(Debug, Default, PartialEq, Clone)]
struct StructType<T: TraitCanal> {
    data: u8,
    age: Vec<T>,
}

trait TraitCanal {
    fn get(&self) -> u8;
}

impl<T> StructType<T> 
where T: TraitCanal + std::marker::Sized{
    fn get(&self) -> u8 {
        println!("impl TraitCanal<T> for AllType");
        self.data
    }
}


impl<AllStruct> TraitCanal for AllStruct {
    fn get(&self) -> u8 {
        println!("impl TraitCanal<T> for AllType");
        0
    }
}


fn get_static<Type, U>(typ: Type) -> u8
where
    Type: TraitCanal,
    Type: std::ops::Deref<Target = U>,
    U: TraitCanal + ?Sized,
{
    println!("get_static()");
    typ.get()
}

// clear && cargo run --bin struct-trait-generics
fn main() {
    //let instance: StructType<dyn TraitCanal> = Default::default();
    //assert_eq!(0, get_static(&instance));
    //println!("");
    //let instance_box_type: Box<StructType<TraitCanal>> = Box::new(instance);
    //assert_eq!(0, get_static(instance_box_type));
}
