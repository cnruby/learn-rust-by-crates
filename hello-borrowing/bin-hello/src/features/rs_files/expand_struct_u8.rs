

pub const STRUCT_U8_OK :&str = r#"#![allow(unused_variables)]
pub fn main() {
    #[derive(Clone, Copy)]
    struct Struct(u8);
    let instance: Struct = Struct(42u8);
    let clone_instance = instance.clone();
    let copy_instance = instance;
    let use_instance = instance;
}
"#;

pub const STRUCT_U8_CP :&str = r#"#![allow(unused_variables)]
pub fn main() {
    #[derive(Clone)]
    struct Struct(u8);
    let instance = Struct(42u8);
    let clone_instance = instance.clone();
    let copy_instance = instance;
    let use_copy_instance = copy_instance;
}
"#;

pub const STRUCT_U8_ERR_01 :&str = r#"#![allow(unused_variables)]
pub fn main() {
    #[derive(Clone)]
    struct Struct(u8);
    let instance = Struct(42u8);
    let clone_instance = instance.clone();
    let copy_instance = instance;
    let use_instance = instance;
}
"#;

pub const STRUCT_U8_ERR_02 :&str = r#"#![allow(unused_variables)]
pub fn main() {
    #[derive(Clone)]
    struct Struct(u8);
    let instance = Struct(42u8);
    let clone_instance = instance.clone();
    let copy_instance = instance;
    let use_copy_instance = copy_instance;
    let use_instance = instance;
}
"#;

pub const STRUCT_U8_ERR_03 :&str = r#"#![allow(unused_variables)]
pub fn main() {
    // error[E0277]: the trait bound `struct_u8::main::Struct: std::clone::Clone` is not satisfied
    #[derive(Copy)]
    struct Struct(u8);
    let instance = Struct(42u8);
    let clone_instance = instance.clone();
    let copy_instance = instance;
    let use_instance = instance;
}
"#;

pub const STRUCT_U8_ERR_10 :&str = r#"#![allow(unused_variables)]
pub fn main() {
    struct Struct(u8);
    let instance = Struct(42u8);
    let copy_instance = instance;
    let use_instance = instance;
}
"#;