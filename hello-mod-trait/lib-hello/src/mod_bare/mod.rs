#[derive(Debug, Default, PartialEq)]
pub struct StructType {
    pub data: (u32),
}

impl StructType {
    pub fn get(&self) -> (u32) {
        (self.data)
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct TupleType (pub u32);

impl TupleType {
    pub fn get(&self) -> (u32) {
        (self.0)
    }
}