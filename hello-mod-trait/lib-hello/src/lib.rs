pub mod mod_bare;
pub mod mod_where_fn;
pub mod mod_static_fn;
pub mod mod_dynamic_fn;

pub mod mod_trait {
    #[derive(Debug, Default, PartialEq, Copy, Clone)]
    pub struct StructType {
        pub data: (u32),
    }

    #[derive(Debug, Default, PartialEq, Copy, Clone)]
    pub struct TupleType(pub u32);

    pub trait TraitCanal {
        //fn new(data: u32) -> StructType;
        fn get(&self) -> (u32);
    }

    impl TraitCanal for StructType {
        fn get(&self) -> (u32) {
            println!("impl TraitCanal for StructType");
            self.data
        }
    }

    impl TraitCanal for TupleType {
        fn get(&self) -> (u32) {
            println!("impl TraitCanal for TupleType");
            self.0
        }
    }
}