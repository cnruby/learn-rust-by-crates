pub const CLOSURE_STRING_OK: &str = r#"// File: ./examples/closure_string.rs

fn main() {
    let string_instance: String = "Hello".to_string();

    let greet_friend = |str: &str| println!("{} Friend", str);
    greet_friend(&string_instance);

    println!("{} World!", string_instance);
}
"#;

pub const CLOSURE_STRING_ERR: &str = r#"// File: ./examples/closure_string.rs

fn main() {
    let string_instance: String = "hello".to_string();

    let greet_friend = |string: String| println!("{} Friend", string);
    greet_friend(string_instance);

    println!("{} World!", string_instance);
}
"#;
