pub const MOVE_VEC_OK: &str = r#"#![allow(unused_variables)]

fn main() {
    let v: Vec<u8> = vec![1, 2, 3];
    println!("v is {:p}", &v);

    let z = v.clone();
    println!("z is {:p}", &z);
    println!("v is {:p}", &v);

    let w = v;
    println!("w is {:p}", &w);
}
"#;

pub const MOVE_VEC_ERR: &str = r#"#![allow(unused_variables)]

fn main() {
    let v: Vec<u8> = vec![1, 2, 3];
    println!("v is {:p}", &v);

    let z = v.clone();
    println!("z is {:p}", &z);
    println!("v is {:p}", &v);

    let w = v;  // value moved here
    println!("w is {:p}", &w);
    println!("v is {:p}", &v); // ERROR: value borrowed here after move
}
"#;
