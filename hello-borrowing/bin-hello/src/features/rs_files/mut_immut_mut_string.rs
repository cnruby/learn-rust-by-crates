

pub const MUT_STRING_OK :&str = r#"#![allow(unused_variables)]
pub fn main() {
    fn one(input: &str) -> String {  
        let result = format!("{}{}", input, "?");
        println!("two() input = {:p}", &input);
        result
    }
    
    fn two(input: &str) -> String {
        let result = format!("{}{}", input, "?");
        println!("two() input = {:p}", &input);
        result
    }
    let mut instance = String::new();
    println!("instance = {:p}", &instance);
    instance.push_str("Hello");
    println!("instance = {:p}", &instance);
    instance = one(&instance);
    println!("instance = {:p}", &instance);
    instance = two(&instance);
    println!("instance = {:p}", &instance);
    println!("instance = {:?}", instance);
}
"#;

pub const MUT_STRING_CP :&str = r#"#![allow(unused_variables)]
pub fn main() {
    fn one(mut hello_from: String) -> String {
        hello_from.push('!');
        hello_from
    }
    
    fn two(mut hello_from: String) -> String {
        hello_from.push('?');
        hello_from
    }
    let mut hello_from = String::new();
    hello_from.push_str("Hello");
    hello_from = one(hello_from);
    hello_from = two(hello_from);
    println!("hello from {:?}", hello_from);
}
"#;

pub const MUT_STRING_OKAY :&str = r#"#![allow(unused_variables)]
pub fn main() {
    fn one(input: &str) -> Box<String> {
        let result = format!("{}{}", input, "?");
        Box::new(result)
    }
    fn two(input: &str) -> Box<String> {
        let result = format!("{}{}", input, "!");
        Box::new(result)
    }
    let mut hello_from = String::new();
    hello_from.push_str("Hello");
    let mut mut_ref :&str = &hello_from;
    hello_from = one(mut_ref).to_string();
    mut_ref = &hello_from;
    hello_from = two(mut_ref).to_string();
    println!("hello from {:?}", hello_from);
    let mut_ref :&str = &hello_from;
    hello_from = one(mut_ref).to_string();
    let mut_ref :&str = &hello_from;
    hello_from = two(mut_ref).to_string();
    println!("hello from {:?}", hello_from);
}
"#;

pub const MUT_STRING_OKEY :&str = r#"#![allow(unused_variables)]
pub fn main() {
    fn one(input: &mut String) -> Box<String> {
        input.push('?');
        Box::new(input.to_string())
    }
    fn two(input: &mut String) -> Box<String> {
        input.push('!');
        Box::new(input.to_string())
    }    
    let mut mut_instance = String::new();
    mut_instance.push_str("Hello");
    println!("1. mut_instance = {:?}", mut_instance);
    
    let mut mut_ref :&mut String = &mut mut_instance;
    mut_instance = one(mut_ref).to_string();
    println!("2. mut_instance = {:?}", mut_instance);
    
    mut_ref = &mut mut_instance;
    mut_instance = two(mut_ref).to_string();
    println!("3. mut_instance = {:?}", mut_instance);
}
"#;

pub const MUT_STRING_ERR_01 :&str = r#"#![allow(unused_variables)]
pub fn main() {
    fn one(hello_from: &mut String) -> &String {
        hello_from.push('!');
        hello_from
    }
    
    fn two(hello_from: &mut String) -> &String {
        hello_from.push('?');
        hello_from
    }
    let mut hello_from = String::new();
    hello_from.push_str("Hello");
    let mut_ref :&String = &mut hello_from; //ERROR
    one(mut_ref);
    two(mut_ref);
    println!("hello from {:?}", hello_from);
}
"#;

pub const MUT_STRING_ERR_02 :&str = r#"#![allow(unused_variables)]
pub fn main() {
    
    fn one(input: &str) -> Box<String> {
        let mut buf = String::with_capacity(input.len());
        buf.push_str(input);
        buf.push('!');
        buf.into()
    }
    fn two(input: &str) -> Box<String> {
        let mut buf = String::with_capacity(input.len());
        buf.push_str(input);
        buf.push('!');
        buf.into()
    }
    let mut hello_from = String::new();
    hello_from.push_str("Hello");
    let mut_ref :&str = &hello_from;
    hello_from = one(mut_ref).to_string();
    hello_from = two(mut_ref).to_string();
    println!("hello from {:?}", hello_from);
}
"#;

pub const MUT_STRING_ERR_03 :&str = r#"#![allow(unused_variables)]
pub fn main() {
    
    use std::borrow::Cow;
    fn one<'a>(input: &'a str) -> Cow<'a, str> {
        let mut buf = String::with_capacity(input.len());
        buf.push_str(input);
        buf.push('!');
        buf.into()
    }
    fn two<'a>(input: &'a str) -> Cow<'a, str> {
        let mut buf = String::with_capacity(input.len());
        buf.push_str(input);
        buf.push('!');
        buf.into()
    }    
    let mut hello_from = String::new();
    hello_from.push_str("Hello");
    let mut_ref :&str = &hello_from;
    let fn_one :Cow<'_, str> = one(mut_ref);
    hello_from = fn_one.to_string();
    hello_from = two(mut_ref).to_string();
    
    println!("hello from {:?}", hello_from);
}
"#;

pub const MUT_STRING_ERR_04 :&str = r#"#![allow(unused_variables)]
pub fn main() {
    
    fn one(input: &str) -> Box<String> {
        let result = [input, "!"].join("");
        println!("one() input = {:p}", &input);
        let result = [input, "!"].concat();
        println!("one() input = {:p}", &input);
        Box::new(result)
    }
    fn two(input: &str) -> Box<String> {
        let result = [input, "!"].join("");
        println!("one() input = {:p}", &input);
        let result = [input, "!"].concat();
        println!("one() input = {:p}", &input);
        Box::new(result)
    }    
    let mut hello_from = String::new();
    hello_from.push_str("Hello");
    
    let mut_ref :&str = &hello_from; // OK
    
    hello_from = one(mut_ref).to_string();
    
    hello_from = two(mut_ref).to_string();
    println!("hello from {:?}", hello_from);
}
"#;

pub const MUT_STRING_ERR_05 :&str = r#"#![allow(unused_variables)]
pub fn main() {
    fn one(hello_from: &mut String) -> &String {
        hello_from.push('!');
        hello_from
    }
    
    fn two(hello_from: &mut String) -> &String {
        hello_from.push('?');
        hello_from
    }
    let mut hello_from = String::new();
    hello_from.push_str("Hello");
    let mut_ref :&String = &mut hello_from;  // ERROR
    one(mut_ref);
    two(mut_ref);
    println!("hello from {:?}", hello_from);
}
"#;

pub const MUT_STRING_ERR_06 :&str = r#"#![allow(unused_variables)]
pub fn main() {
    fn one(hello_from: String) -> String {
        hello_from.push('!');
        hello_from
    }
    
    fn two(hello_from: String) -> String {
        hello_from.push('?');
        hello_from
    }
    let mut hello_from = String::new();
    hello_from.push_str("Hello");
    hello_from = one(hello_from);
    hello_from = two(hello_from);
    println!("hello from {:?}", hello_from);
}
"#;