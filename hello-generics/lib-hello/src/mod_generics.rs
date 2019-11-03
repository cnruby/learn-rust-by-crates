#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub struct StructType {
    pub data: (u32),
}

#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub struct TupleType(u32);

pub trait TraitCanal {
    //fn new(data: u32) -> StructType;
    fn get(&self) -> (u32);
}

impl TraitCanal for StructType {
    fn get(&self) -> (u32) {
        println!("impl TraitCanal for StructType");
        (self.data)
    }
}

impl TraitCanal for TupleType {
    fn get(&self) -> (u32) {
        println!("impl TraitCanal for TupleType");
        (self.0)
    }
}

pub trait Trait {
    fn get(&self) -> (u32);
}

impl<T: TraitCanal> Trait for T {
    fn get(&self) -> (u32) {
        println!("impl TraitCanal for AllType");
        self.get()
    }
}