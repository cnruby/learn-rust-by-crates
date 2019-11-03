pub const VEC_FOR_OK: &str = r#"fn main() {
    let a = [1, 2, 3];
    for i in a.iter() {
        print!("{} ", i);
    }
    println!("");
    println!("v = {:?}", a);

    let v = vec![1, 2, 3];
    for i in &v {
        print!("{} ", i);
    }
    println!("");
    println!("v = {:?}", v);
}

"#;

pub const VEC_FOR_ERR: &str = r#"fn main() {
    let a = [1, 2, 3];
    for i in a {
        print!("{} ", i);
    }
    println!("");
    println!("a = {:?}", a);

    let v = vec![1, 2, 3];
    for i in v {
        print!("{} ", i);
    }
    println!("");
    println!("v = {}", v); // error here
}
"#;
