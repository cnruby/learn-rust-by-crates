

pub const BASE_STRING_OK :&str = r#"#![allow(unused_variables)]
pub fn main() {
    fn one(input: &mut String) {
        input.push_str(", world");
        println!("one() input = {:p}", input);
    }
    fn two(input: &mut String) {
        input.push('!');
        println!("two() input = {:p}", input);
    }
 
    let mut instance = String::new();
    println!("instance = {:p}", &instance);
    instance.push_str("Hello");
    let mut_ref = &mut instance;
    println!("mut_ref = {:p}", mut_ref);
    one(mut_ref);
    two(mut_ref);
    
    println!("instance = {:p}", &instance);
    println!("{}", instance);
}
"#;

pub const BASE_STRING_CP :&str = r#"#![allow(unused_variables)]
pub fn main() {
    fn one(input: &str) -> String {        
        println!("input = {:p}", input);
        let result = format!("{}{}", input, "!");
        println!("input = {:p}", input);
        println!("input = {}", input);
        println!("result = {:p}", &result);
        println!("result = {}", result);
        result
    }
    fn two(input: &str) -> String {
        println!("input = {:p}", input);
        let result = format!("{}{}", input, "?");
        println!("input = {:p}", input);
        println!("input = {}", input);
        println!("result = {:p}", &result);
        println!("result = {}", result);
        result
    }
 
    let mut instance = String::new();
    println!("instance = {:p}", &instance);
    instance.push_str("Hello");
    let mut mut_ref :&str = &instance;
    println!();
    println!("mut_ref = {:p}", mut_ref);
    instance = one(mut_ref);    
    
    mut_ref = &instance;
    println!();
    println!("mut_ref = {:p}", mut_ref);
    instance = two(mut_ref);
    println!();
    println!("instance = {:p}", &instance);
    println!("{}", instance);
}
"#;

pub const BASE_STRING_OKAY :&str = r#"#![allow(unused_variables)]
pub fn main() {
    fn one(mut input: String) {
        input.push_str(", world");        
        println!("input = {:p}", &input);
        println!("input = {}", input);
    }
 
    let mut instance = String::new();
    println!("instance = {:p}", &instance);
    instance.push_str("Hello");
    one(instance);
}
"#;

pub const BASE_STRING_OKEY :&str = r#"#![allow(unused_variables)]
pub fn main() {
    fn one(x: &str, y: &String) {
        x.to_ascii_uppercase();
    }
    let first_str = "Hello";
    let second_str = "World".to_string();
    one(first_str, &second_str);
    
}
"#;

pub const BASE_STRING_ERR_04 :&str = r#"#![allow(unused_variables)]
pub fn main() {
    fn one(mut input: String) {
        input.push_str(", world");
        println!("input = {:p}", &input);
    }
    let mut instance = String::new();
    println!("instance = {:p}", &instance);
    instance.push_str("Hello");
    one(instance);
    
    println!("{}", instance);
}
"#;

pub const BASE_STRING_ERR_05 :&str = r#"#![allow(unused_variables)]
pub fn main() {
    fn one(input: &String) {
        input.push_str(", world");
        println!("input = {:p}", input);
    }
    
    fn two(input: &String) {
        input.push('!');
        println!("input = {:p}", input);
    }
    
    let mut instance = String::new();
    println!("instance = {:p}", &instance);
    
    instance.push_str("Hello");
    let mut_ref = &mut instance;
    println!("mut_ref = {:p}", mut_ref);
    
    one(mut_ref);
    two(mut_ref);
    
    println!("instance = {:p}", &instance);
    println!("{}", instance);
    
}
"#;

pub const BASE_STRING_ERR_06 :&str = r#"#![allow(unused_variables)]
pub fn main() {
    fn one(input: String) {
        input.push_str(", world");
        println!("input = {:p}", &input);
    }
    let mut instance = String::new();
    println!("instance = {:p}", &instance);
    instance.push_str("Hello");
    one(instance);
}
"#;