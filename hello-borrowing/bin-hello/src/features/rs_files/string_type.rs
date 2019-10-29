pub const STRING_TYPE_OK :&str = r#"#![allow(unused_variables)]

fn main() {
    let instance = String::from("hello");
    println!("{}", instance);
    
    let copy_instance = instance;
    
}
"#;

pub const STRING_TYPE_ERR :&str = r#"#![allow(unused_variables)]

fn main() {
    let instance = String::from("hello");
    println!("{}", instance);
    
    let copy_instance = instance;
    
    println!("{}", instance);
}
"#;
