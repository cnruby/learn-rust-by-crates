pub const EXPAND_STRUCT_OK: &str = r#"fn main() {
    #[derive(Clone, Copy)]
    struct Struct(u8);

    let instance = Struct(42u8);
    let cln_instance = instance.clone(); // Clone
    let cp_instance = instance; // Copy
    dbg!(instance.0, cp_instance.0, cln_instance.0);
}
"#;

pub const EXPAND_STRUCT_ERR: &str = r#"fn main() {
    #[derive(Clone)]
    struct Struct(u8);

    let instance = Struct(42u8);
    let cln_instance = instance.clone(); // Clone
    let cp_instance = instance; // Copy
    dbg!(instance.0, cp_instance.0, cln_instance.0);
}
"#;
