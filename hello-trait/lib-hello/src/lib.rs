#[derive(Debug, PartialEq)]
pub struct StructType {
    data: u32,
}

// Defining a trait for any type
pub trait TraitCanal {
    //fn new(data: u32) -> StructType;
    fn get_data(&self) -> u32;
}

// Defining a trait for any type
pub trait TraitKanal {
    //fn new(data: u32) -> StructType;
    fn set_data(&mut self, data: &u32);
}

// Implementing a trait for a type
impl TraitCanal for StructType {
    fn get_data(&self) -> u32 {
        self.data
    }
}

// Implementing a trait for a type
impl TraitKanal for StructType {
    fn set_data(&mut self, data: &u32) {
        self.data = *data;
    }
}

// impl AllTrait for StructType {
impl StructType {
    pub fn new(data: u32) -> StructType {
        dbg!("impl StructType: new");
        StructType { data: data }
    }

    pub fn get_data_for_all(&self) -> u32 {
        self.data
    }

    pub fn set_data_for_all(&mut self, data: &u32) {
        self.data = *data;
    }
}
