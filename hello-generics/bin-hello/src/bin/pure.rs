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

impl TraitCanal for &StructType {
    fn get(&self) -> u32 {
        println!("impl TraitCanal for &StructType");
        self.data
    }
}

impl TraitCanal for Box<StructType> {
    fn get(&self) -> u32 {
        println!("impl TraitCanal for Box<StructType>");
        self.data
    }
}

fn get_static<Type, U>(typ: Type) -> u32
where
    Type: TraitCanal,
    Type: std::ops::Deref<Target = U>,
    U: TraitCanal + ?Sized,
{
    println!("get_static()");
    typ.get()
}

fn main() {
    let instance: StructType = Default::default();
    assert_eq!(0, get_static(&instance));
    println!("");
    let instance_box_type: Box<StructType> = Box::new(instance);
    assert_eq!(0, get_static(instance_box_type));
}
