pub const CLOSURE_VEC_OK: &str = r#"fn main() {
    let instance = vec![1, 2, 3];
    println!("The variable instance before borrowing: {:?}", instance);

    let equal_to_val = move |z| z == instance;


    let y = vec![1, 2, 3];
    assert!(equal_to_val(y));
}
"#;

pub const CLOSURE_VEC_ERR: &str = r#"fn main() {
    let instance = vec![1, 2, 3];
    println!("The variable instance before borrowing: {:?}", instance);

    let equal_to_val = move |z| z == instance;

    println!("The variable instance after borrowing: {:?}", instance);

    let y = vec![1, 2, 3];
    assert!(equal_to_val(y));
}
"#;
