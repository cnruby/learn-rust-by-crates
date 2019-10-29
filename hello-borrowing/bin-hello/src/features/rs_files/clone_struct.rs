pub const CLONE_STRUCT_OK :&str = r#"#![allow(unused_variables)]

fn main() {
    use std::ptr::eq;

    let a = String::new();
    let b = a.clone();
    println!("{:?} {:?}", a, b);
    println!("{:p} {:p}", &a, &b);

    assert_eq!(a, b);
    assert!(!eq(&a, &b));
}
"#;

pub const CLONE_STRUCT_ERR :&str = r#"#![allow(unused_variables)]

fn main() {
    use std::ptr::eq;

    let a = String::new();
    let b = a.clone();
    let c = a;
    println!("{:?} {:?}", a, b);
    println!("{:p} {:p}", &a, &b);

    assert_eq!(a, b);
    assert!(!eq(&a, &b));
}
"#;
