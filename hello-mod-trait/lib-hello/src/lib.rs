pub mod mod_bare;
pub mod mod_where_fn;
pub mod mod_static_fn;
pub mod mod_box_static_fn;
pub mod mod_dynamic_fn;
pub mod mod_box_dynamic_fn;

pub mod mod_trait {
    #[derive(Debug, Default, PartialEq, Copy, Clone)]
    pub struct StructType {
        data: (u32),
    }

    #[derive(Debug, Default, PartialEq, Copy, Clone)]
    pub struct TupleType(pub u32);

    pub trait TraitCanal {
        fn get_object(&self) -> Self where Self: Sized;  // For keyword `dyn`
        //fn get_object(&self) -> Self;                  // E0038 For keyword `dyn`; OK for static functions
        fn get_tuple(&self) -> (u32);
    }

    impl TraitCanal for StructType {
        fn get_object(&self) -> Self {
            StructType{data: self.data}
        }

        fn get_tuple(&self) -> (u32) {
            println!("impl TraitCanal for StructType");
            (self.data)
        }
    }

    impl TraitCanal for TupleType {
        fn get_object(&self) -> Self {
            TupleType(self.0)
        }

        fn get_tuple(&self) -> (u32) {
            println!("impl TraitCanal for TupleType");
            (self.0)
        }
    }

    impl StructType {
        pub fn new(_data: u32) -> Self {
            StructType{data: _data}
        }
    }

    impl TupleType {
        pub fn new(_data: u32) -> Self {
            TupleType(_data)
        }
    }
}