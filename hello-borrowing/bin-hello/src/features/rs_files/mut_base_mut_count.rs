

pub const MUT_COUNT_OK :&str = r#"#![allow(unused_mut)]
#![allow(unused_variables)]
pub fn main() {
    let instance = vec![33u8, 42];
    let first_ref = &instance; // immutable reference
    let second_ref = &instance; // immutable reference
    println!("{:?} {:?}", first_ref, second_ref);
    let mut instance = vec![33, 42u8];
    let first_mut_ref = &instance; // immutable reference
    let second_mut_ref = &instance; // immutable reference
    println!("{:?} {:?}", first_mut_ref, second_mut_ref);
}
"#;

pub const MUT_COUNT_ERR_01 :&str = r#"#![allow(unused_mut)]
#![allow(unused_variables)]
pub fn main() {
    let mut instance = vec![33u8, 42];
    let first_mut_ref = &mut instance; // mutable reference
    let second_mut_ref = &mut instance; // mutable reference
    println!("{:?} {:?}", first_mut_ref, second_mut_ref);
}
"#;

pub const MUT_COUNT_ERR_02 :&str = r#"#![allow(unused_mut)]
#![allow(unused_variables)]
pub fn main() {
    let mut instance = vec![33, 42u8];
    let first_immut_ref = &instance; // immutable reference
    let second_mut_ref = &mut instance; // mutable reference
    println!("{:?}", first_immut_ref);
}
"#;

pub const MUT_COUNT_ERR_03 :&str = r#"#![allow(unused_mut)]
#![allow(unused_variables)]
pub fn main() {
    let mut instance = vec![33u8, 42];
    let first_mut_ref = &mut instance; // mutable reference
    let second_immut_ref = &instance; // immutable reference
    println!("{:?}", first_mut_ref);
}
"#;