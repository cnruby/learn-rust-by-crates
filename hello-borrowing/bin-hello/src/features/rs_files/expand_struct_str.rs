

pub const STRUCT_STR_OK :&str = r#"#![allow(unused_variables)]
pub fn main() {
    #[derive(Clone, Copy)]
    struct Struct<'cn>(&'cn str);
    let instance: Struct = Struct("Hello");
    let clone_instance = instance.clone();
    let copy_instance = instance;
    let use_instance = instance;
}
"#;

pub const STRUCT_STR_CP :&str = r#"#![allow(unused_variables)]
pub fn main() {
    #[derive(Clone)]
    struct Struct<'cn>(&'cn str);
    let instance: Struct = Struct("Hello");
    let clone_instance = instance.clone();
    let copy_instance = instance;
    let use_copy_instance = copy_instance;
}
"#;

pub const STRUCT_STR_ERR_07 :&str = r#"#![allow(unused_variables)]
pub fn main() {
    #[derive(Clone)]
    struct Struct<'cn>(&'cn str);
    let instance: Struct = Struct("Hello");
    let clone_instance = instance.clone();
    let copy_instance = instance;
    let use_instance = instance;
}
"#;

pub const STRUCT_STR_ERR_08 :&str = r#"#![allow(unused_variables)]
pub fn main() {
    #[derive(Clone)]
    struct Struct<'cn>(&'cn str);
    let instance: Struct = Struct("Hello");
    let clone_instance = instance.clone();
    let copy_instance = instance;
    let use_copy_instance = copy_instance;
    let use_instance = instance;
}
"#;

pub const STRUCT_STR_ERR_09 :&str = r#"#![allow(unused_variables)]
pub fn main() {
    // error[E0277]: the trait bound `struct_str::main::Struct<'cn>: std::clone::Clone` is not satisfied
    #[derive(Copy)]
    struct Struct<'cn>(&'cn str);
    let instance: Struct = Struct("Hello");
    let clone_instance = instance.clone();
    let copy_instance = instance;
    let use_instance = instance;
}
"#;