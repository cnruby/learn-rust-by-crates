

pub const BASE_STR_OK :&str = r#"#![allow(unused_variables)]
pub fn main() {
    fn one<'de>(x: &'de str, y: &'de str) -> &'de str {
        println!("x = {:p}", x);
        println!("y = {:p}", y);
        if x.is_empty() {
            x
        } else {
            y
        }
    }
    let first_str = "";
    let second_str = "World";
    println!("first_str = {:p}", first_str);
    println!("second_str = {:p}", second_str);
    let result = one(first_str, second_str);
    println!();
    println!("first_str = {:p}", first_str);
    println!("second_str = {:p}", second_str);
    println!("result = {:p}", result);
    println!("result = {}", result);
}
"#;

pub const BASE_STR_CP :&str = r#"#![allow(unused_variables)]
pub fn main() {
    fn one(x: &str, y: &str) {
        println!("x = {:p}", x);
        println!("y = {:p}", y);
    }
    
    let hello = "Hello";
    let world = "World";
    println!("hello = {:p}", hello);
    println!("world = {:p}", world);
    one(hello, world);
    
}
"#;

pub const BASE_STR_OKAY :&str = r#"#![allow(unused_variables)]
pub fn main() {
    fn one<'cn>(x: &'cn str, y: &str) -> &'cn str {
        x
    }
    let first_str = "Hello";
    let second_str = "World";
    let result = one(first_str, second_str);
    println!("result = {}", result);
}
"#;

pub const BASE_STR_OKEY :&str = r#"#![allow(unused_variables)]
pub fn main() {
    fn one(input: &str) {
        println!("one() input = {:p}", input);
        let ret_input = input.to_ascii_uppercase();
        println!("one() ret_input = {:p}", &ret_input);
        println!("one() ret_input = {}", ret_input);
        
        println!("one() input = {:p}", input);
        println!("one() input = {}", input);
    }
 
    let mut instance = String::new();
    instance.push_str("Hello");
    println!("after instance change = {:p}", &instance);
    println!();
    let immut_ref :&str = &instance;
    println!("before call one instance = {:p}", immut_ref);
    one(immut_ref);
    println!("after call one instance = {:p}", immut_ref);
    println!("instance = {}", instance);
}
"#;

pub const BASE_STR_ERR_07 :&str = r#"#![allow(unused_variables)]
pub fn main() {
    fn one(x: &str, y: &str) -> &str {
        x
    }
    let first_str = "Hello";
    let second_str = "World";
    let result = one(first_str, second_str);
    println!("result = {}", result);
}
"#;

pub const BASE_STR_ERR_08 :&str = r#"#![allow(unused_variables)]
pub fn main() {
    fn one<'cn>(x: &'cn str, y: &str) -> &'cn str {
        y
    }
    let first_str = "Hello";
    let second_str = "World";
    let result = one(first_str, second_str);
    println!("result = {}", result);
}
"#;

pub const BASE_STR_ERR_09 :&str = r#"#![allow(unused_variables)]
pub fn main() {
    fn one(x: &str, y: &String) -> &str {
        x
    }
    let first_str = "Hello";
    let second_str = "World".to_string();
    let result = one(first_str, second_str);
    println!("result = {}", result);
}
"#;

pub const BASE_STR_ERR_10 :&str = r#"#![allow(unused_variables)]
pub fn main() {
    fn one() -> &'de str {
        "Hello"
    }
    let result = one();
    println!("result = {}", result);
}
"#;