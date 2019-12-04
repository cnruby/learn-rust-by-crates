

pub const KW_MOVE_OK :&str = r#"pub fn main() {
    let instance = vec![1, 2, 3];
    (|| (instance.len()))();
    println!("{:?}", instance); // ok; x was not moved
}
"#;

pub const KW_MOVE_CP :&str = r#"pub fn main() {
    let instance = vec![1, 2, 3];
    (|| {
        (&instance);
    })();
    println!("{:?}", instance);
}
"#;

pub const KW_MOVE_ERR_02 :&str = r#"pub fn main() {
    let instance = vec![1, 2, 3];
    (move || {
        (&instance);
    })();
    println!("{:?}", instance);
}
"#;