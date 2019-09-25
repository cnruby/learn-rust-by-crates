pub struct MyType {
    pub data: u32,
}

// Defining an interface
pub trait MyTrait {
    fn foo(&self) -> u32;
}

// Implementing an interface
impl MyTrait for MyType {
    fn foo(&self) -> u32 {
        self.data
    }
}
