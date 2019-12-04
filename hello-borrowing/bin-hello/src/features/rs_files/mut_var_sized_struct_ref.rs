

pub const STRUCT_REF_OK :&str = r#"#![allow(unused_variables)]
pub fn main() {
    struct Struct(u8);
    let mut instance = Struct(42u8);
    let ref_mut_instance = &mut instance;
    ref_mut_instance.0 = 33;
    println!("ref_mut_instance.data = {}", ref_mut_instance.0);
    println!("instance.data = {}", instance.0);
}
"#;

pub const STRUCT_REF_ERR_08 :&str = r#"#![allow(unused_variables)]
pub fn main() {
    struct Struct(u8);
    let mut instance = Struct(42u8);
    let new_instance = instance;
    instance.0 = 33;
    println!("instance.data = {}", instance.0);
}
"#;