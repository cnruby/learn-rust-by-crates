

pub const IMMUT_STRING_OK :&str = r#"pub fn main() {
    let string_instance: String = "Hello".to_string();
    println!("Before fn = {:p}", &string_instance);
    let closure_instance = |hello: &str| {
        println!("{} Friend!", hello);
        println!("Inside fn = {:p}", &hello);
    };
    closure_instance(&string_instance);
    println!("After fn = {:p}", &string_instance);
    println!("{} World!", string_instance);
}
"#;

pub const IMMUT_STRING_ERR_01 :&str = r#"pub fn main() {
    let string_instance: String = "Hello".to_string();
    let closure_instance = |hello: String| println!("{} Friend!", hello);
    closure_instance(string_instance);
    println!("{} World!", string_instance);
}
"#;