#[derive(Debug, PartialEq, Default)]
pub struct StructType {
    data: u32,
}

pub trait TraitCanal {
    fn get_data(&self) -> u32;
}

pub trait TraitKanal {
    fn set_data(&mut self, data: &u32);
}

impl TraitCanal for StructType {
    fn get_data(&self) -> u32 {
        self.data
    }
}

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
