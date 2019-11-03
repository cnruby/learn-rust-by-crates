pub const DBG_MARCO_OK: &str = r#"fn main() {
    let str = format!("{}", "Hello");
    dbg!(str);
}
"#;

pub const DBG_MARCO_ERR: &str = r#"fn main() {
    let str = format!("{}", "Hello");
    dbg!(str);
    dbg!(&str);
}
"#;
