

pub const STRUCT_STRING_OK :&str = r#"#![allow(unused_variables)]
pub fn main() {
    #[derive(Clone)]
    struct Struct(String);
    let instance = Struct(String::from("Hello"));
    let clone_instance = instance.clone();
    let copy_instance = instance;
    let use_copy_instance = copy_instance;
}
"#;

pub const STRUCT_STRING_ERR_04 :&str = r#"#![allow(unused_variables)]
pub fn main() {
    #[derive(Clone, Copy)]
    struct Struct(String);
    let instance = Struct(String::from("Hello"));
    let clone_instance = instance.clone();
    let copy_instance = instance;
    let use_instance = instance;
}
"#;

pub const STRUCT_STRING_ERR_05 :&str = r#"#![allow(unused_variables)]
pub fn main() {
    #[derive(Clone)]
    struct Struct(String);
    let instance = Struct(String::from("Hello"));
    let clone_instance = instance.clone();
    let copy_instance = instance;
    let use_instance = instance;
}
"#;

pub const STRUCT_STRING_ERR_06 :&str = r#"#![allow(unused_variables)]
pub fn main() {
    #[derive(Copy)]
    struct Struct(String);
    let instance = Struct(String::from("Hello"));
    let clone_instance = instance.clone();
    let copy_instance = instance;
    let use_instance = instance;
}
"#;