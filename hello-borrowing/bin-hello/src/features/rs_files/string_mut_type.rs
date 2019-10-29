pub const STRING_MUT_TYPE_OK :&str = r#"#![allow(unused_variables)]

fn main() {
    let mut mut_instance = String::from("hello");
    mut_instance.push_str(", world!");
    println!("{}", mut_instance);
    
    // The variable `mut_instance` begin to move here
    let copy_mut_instance = mut_instance;
    // The variable `mut_instance` moved here
    
    // The variable `mut_instance` borrowed here after move
    //println!("{}", mut_instance);
}
"#;

pub const STRING_MUT_TYPE_ERR :&str = r#"#![allow(unused_variables)]

fn main() {
    let mut mut_instance = String::from("hello");
    mut_instance.push_str(", world!");
    println!("{}", mut_instance);
    
    // The variable `mut_instance` begin to move here
    let copy_mut_instance = mut_instance;
    // The variable `mut_instance` moved here
    
    // ERROR: The variable `mut_instance` borrowed here after move
    println!("{}", mut_instance);
}
"#;