

pub const REF_STR_OK :&str = r#"#![allow(unused_variables)]
pub fn main() {
    fn one(input: &str) -> String {  
        let result = format!("{}{}", input, ", world");
        println!("one() input = {:p}", input);
        result
    }
    
    fn two(input: &str) -> String {
        let result = format!("{}{}", input, "!");
        println!("two() input = {:p}", input);
        result
    }
    let mut instance = String::new();
    println!("instance = {:p}", &instance);
    instance.push_str("Hello");
    println!();
    let mut mut_immut_ref :&str = &instance;
    println!("before call one() mut_immut_ref = {:p}", mut_immut_ref);
    let tmp_instance = one(mut_immut_ref);
    println!("after call one() mut_immut_ref = {:p}", mut_immut_ref);
    instance = tmp_instance;
    println!("instance = {:?}", instance);
    println!();
    mut_immut_ref = &instance;
    println!("mut_immut_ref = {:p}", mut_immut_ref);
    instance = two(mut_immut_ref);
    println!("instance = {:?}", instance);
    println!();
    println!("instance = {:p}", &instance);
}
"#;

pub const REF_STR_CP :&str = r#"#![allow(unused_variables)]
pub fn main() {
    use std::borrow::Cow;
    fn one<'a>(input: &'a str) -> Cow<'a, str> {
        let mut buf = String::with_capacity(input.len());
        buf.push_str(input);
        buf.push_str(", world");
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
    let mut_ref :&str = &hello_from;
    hello_from = two(mut_ref).to_string();
    
    println!("hello from {:?}", hello_from);
}
"#;

pub const REF_STR_OKAY :&str = r#"#![allow(unused_variables)]
pub fn main() {
    fn one(input: &str) -> Box<String> {
        let result = format!("{}{}", input, ", world");
        Box::new(result)
    }
    fn two(input: &str) -> Box<String> {
        let result = format!("{}{}", input, "!");
        Box::new(result)
    }
    let mut hello_from = String::new();
    hello_from.push_str("Hello");
    let mut_ref :&str = &hello_from;
    hello_from = one(mut_ref).to_string();
    let mut_ref :&str = &hello_from;
    hello_from = two(mut_ref).to_string();
    println!("hello from {:?}", hello_from);
    let mut mut_ref :&str = &hello_from;
    hello_from = one(mut_ref).to_string();
    mut_ref = &hello_from;
    hello_from = two(mut_ref).to_string();
    println!("hello from {:?}", hello_from);
}
"#;

pub const REF_STR_OKEY :&str = r#"#![allow(unused_variables)]
pub fn main() {
}
"#;

pub const REF_STR_ERR_01 :&str = r#"#![allow(unused_variables)]
pub fn main() {
    fn one(input: &str) -> Box<String> {
        let result = format!("{}{}", input, ", world");
        Box::new(result)
    }
    fn two(input: &str) -> Box<String> {
        let result = format!("{}{}", input, "!");
        Box::new(result)
    }
    let mut hello_from = String::new();
    hello_from.push_str("Hello");
    let mut_ref :&str = &hello_from;
    hello_from = one(mut_ref).to_string();
    hello_from = two(mut_ref).to_string();
    println!("hello from {:?}", hello_from);
}
"#;

pub const REF_STR_ERR_02 :&str = r#"#![allow(unused_variables)]
pub fn main() {
    
}
"#;

pub const REF_STR_ERR_03 :&str = r#"#![allow(unused_variables)]
pub fn main() {
    
    fn manipulate(input: &mut String) {                                                                                                                                                                                                          
        input.push('!');
      }
 
    let mut instance = String::new();
    instance.push_str("Hello");
    let mut_ref = &mut instance;
    manipulate(mut_ref);          
    println!("{}", instance);    
}
"#;