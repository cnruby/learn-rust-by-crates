pub mod mod_trait {
    pub struct StructType {
        pub data: u32,
    }

    // Defining a trait for any type
    pub trait CanalTrait {
        fn foo(&self) -> u32;
    }

    // Implementing a trait for a type
    impl CanalTrait for StructType {
        fn foo(&self) -> u32 {
            self.data
        }
    }
}

pub mod mod_bare {
    pub struct StructType {
        pub data: u32,
    }

    impl StructType {
        pub fn foo(&self) -> u32 {
            self.data
        }
    }
}
