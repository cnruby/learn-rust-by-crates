

pub const STRING_REFS_OK :&str = r#"#![allow(unused_variables)]
#![allow(unused_mut)]
pub fn main() {
    let mut instance = String::new();
    instance.push_str("hello");
    let mut_ref :&mut String = &mut instance;
    mut_ref.push_str(" world");
    let immut_ref :&String = mut_ref;
    println!("immut_ref = {}", immut_ref);
    println!("mut_ref = {}", mut_ref);
    mut_ref.make_ascii_uppercase(); // immut_ref is moved after here
    println!("mut_ref = {}", mut_ref);
    instance.push('!'); // mut_ref is moved after here
    println!("instance = {}", instance);
}
"#;

pub const STRING_REFS_ERR_04 :&str = r#"#![allow(unused_variables)]
#![allow(unused_mut)]
pub fn main() {
    let mut instance = String::new();
    instance.push_str("hello");
    let mut_ref :&mut String = &mut instance;
    mut_ref.push_str(" world");
    let immut_ref :&String = mut_ref;
    println!("immut_ref = {}", immut_ref);
    println!("mut_ref = {}", mut_ref);
    mut_ref.make_ascii_uppercase(); // immut_ref is moved after here
    println!("immut_ref = {}", immut_ref); // ERROR
    println!("mut_ref = {}", mut_ref);
    instance.push('!'); // mut_ref is moved after here
    println!("instance = {}", instance);
}
"#;

pub const STRING_REFS_ERR_05 :&str = r#"#![allow(unused_variables)]
#![allow(unused_mut)]
pub fn main() {
    
    let mut instance = String::new();
    instance.push_str("hello");
    let mut_ref :&mut String = &mut instance;
    mut_ref.push_str(" world");
    let immut_ref :&String = mut_ref;
    println!("immut_ref = {}", immut_ref);
    println!("mut_ref = {}", mut_ref);
    mut_ref.make_ascii_uppercase(); // immut_ref is moved after here
    println!("mut_ref = {}", mut_ref);
    instance.push('!'); // mut_ref is moved after here
    println!("mut_ref = {}", mut_ref);  // ERROR
    println!("instance = {}", instance);
}
"#;

pub const STRING_REFS_ERR_06 :&str = r#"#![allow(unused_variables)]
#![allow(unused_mut)]
pub fn main() {
    let mut instance = String::new();
    instance.push_str("hello");
    let mut_ref :&mut String = &mut instance;
    mut_ref.push_str(" world");
    let immut_ref :&String = &instance;
    println!("immut_ref = {}", immut_ref);
    println!("mut_ref = {}", mut_ref);  // ERROR
    mut_ref.make_ascii_uppercase(); // immut_ref is moved after here
    println!("mut_ref = {}", mut_ref);
    instance.push('!'); // mut_ref is moved after here
    println!("instance = {}", instance);
}
"#;