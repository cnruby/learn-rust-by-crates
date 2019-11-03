pub const NEW_VAR_OK: &str = r#"#![allow(unused_variables)]

fn main() {
    struct Struct(u8);

    let mut instance = Struct(42u8);
    instance.0 = 33;
    println!("instance.data = {}", instance.0);
}

"#;

pub const NEW_VAR_ERR: &str = r#"#![allow(unused_variables)]

fn main() {
    struct Struct(u8);

    let mut instance = Struct(42u8);
    let new_instance = instance;
    instance.0 = 33;
    println!("instance.data = {}", instance.0);
}
"#;
