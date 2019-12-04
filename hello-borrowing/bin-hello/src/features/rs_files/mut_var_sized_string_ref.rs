

pub const STRING_REF_OK :&str = r#"#![allow(unused_variables)]
pub fn main() {
    let mut mut_instance = String::from("Hello");
    mut_instance.push_str(", world");
    println!("1. use mut_instance = {}", mut_instance);
    let immut_ref = &mut_instance;
    println!("1. use immut_ref = {}", immut_ref);
    println!("2. use mut_instance = {}", mut_instance);
    println!("2. use immut_ref = {}", immut_ref);
}
"#;

pub const STRING_REF_ERR_01 :&str = r#"#![allow(unused_variables)]
pub fn main() {
    let mut mut_instance = String::from("hello");
    mut_instance.push_str(", world!");
    println!("1. use mut_instance = {}", mut_instance);
    let copy_mut_instance = mut_instance;
    println!("2. use mut_instance = {}", mut_instance); //ERROR
}
"#;

pub const STRING_REF_ERR_02 :&str = r#"#![allow(unused_variables)]
pub fn main() {
    let mut mut_instance = String::from("Hello");
    mut_instance.push_str(", world");
    println!("1. use mut_instance = {}", mut_instance);
    let immut_ref = &mut_instance;
    println!("1. use immut_ref = {}", immut_ref);
    mut_instance.push_str("!");
    println!("2. use immut_ref = {}", immut_ref); //ERROR
    println!("2. use mut_instance = {}", mut_instance);
}
"#;

pub const STRING_REF_CP :&str = r#"#![allow(unused_variables)]
pub fn main() {
    let mut mut_instance = String::from("Hello");
    mut_instance.push_str(", world");
    println!("1. use mut_instance = {}", mut_instance);
    let mut_ref = &mut mut_instance;
    println!("1. use mut_ref = {}", mut_ref);
    mut_ref.push_str("!");
    println!("2. use mut_ref = {}", mut_ref);
    mut_instance.make_ascii_uppercase();
    println!("2. use mut_instance = {}", mut_instance);
}
"#;

pub const STRING_REF_ERR_03 :&str = r#"#![allow(unused_variables)]
pub fn main() {
    let mut mut_instance = String::from("Hello");
    mut_instance.push_str(", world");
    println!("1. use mut_instance = {}", mut_instance);
    let mut_ref = &mut mut_instance;
    println!("1. use mut_ref = {}", mut_ref);
    mut_ref.push_str("!");
    println!("2. use mut_ref = {}", mut_ref);
    mut_instance.make_ascii_uppercase();
    println!("3. use mut_ref = {}", mut_ref); // ERROR
    println!("2. use mut_instance = {}", mut_instance);
}
"#;