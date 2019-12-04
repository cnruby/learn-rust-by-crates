

pub const STRING_STR_OK :&str = r#"pub fn main() {
    let instance = String::from("Hello");
    let raw_instance = instance.as_str();
    println!("raw_instance = {:p}", raw_instance);
    let borrow_instance: &str = &instance;
    println!("borrow_instance = {:p}", borrow_instance);
    println!("{}", instance);
    println!("{}", borrow_instance);
}
"#;

pub const STRING_STR_ERR_01 :&str = r#"pub fn main() {
    let instance = String::from("hello");
    let copy_instance = instance;
    println!("{}", instance);
    println!("{}", copy_instance);
}
"#;