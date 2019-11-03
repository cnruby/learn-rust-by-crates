pub const MOVE_TUPLE_OK: &str = r#"#![allow(unused_variables)]

fn main() {
    #[derive(Debug, Clone, Copy)]
    struct Tuple(u8);
    let v = Tuple(42);
    println!("v is {:p}", &v);

    let z = v.clone();
    println!("z is {:p}", &z);
    println!("v is {:p}", &v);

    let w = v;
    println!("w is {:p}", &w);
    println!("v is {:p}", &v);
}
"#;

pub const MOVE_TUPLE_ERR: &str = r#"#![allow(unused_variables)]

fn main() {
    #[derive(Debug, Clone)]
    struct Tuple(u8);
    let v = Tuple(42);
    println!("v is {:p}", &v);

    let z = v.clone();
    println!("z is {:p}", &z);
    println!("v is {:p}", &v); 

    let w = v;  // value moved here
    println!("w is {:p}", &w);
    println!("v is {:p}", &v);  // ERROR: value borrowed here after
}
"#;
