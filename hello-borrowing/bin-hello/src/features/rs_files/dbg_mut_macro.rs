

pub const MUT_MACRO_OK :&str = r#"pub fn main() {
    let mut string :String = format!("{}", "Hello");
    string.push('!');
    let ref_string = &string;
    dbg!(ref_string);
    dbg!(ref_string);
}
"#;

pub const MUT_MACRO_CP :&str = r#"pub fn main() {
    let string :String = format!("{}", "Hello");
    dbg!(std::mem::size_of_val(&string));
    dbg!(&string);
    dbg!(string);
}
"#;

pub const MUT_MACRO_ERR_01 :&str = r#"pub fn main() {
    let string = format!("{}", "Hello");
    dbg!(string);
    dbg!(&string);
}
"#;

pub const MUT_MACRO_ERR_02 :&str = r#"pub fn main() {
    let string = format!("{}", "Hello");
    dbg!(string);
    dbg!(string);
}
"#;

pub const MUT_MACRO_ERR_03 :&str = r#"pub fn main() {
    dbg!("");
    let mut string = format!("{}", "Hello");
    string.push('!');
    let ref_mut_string = &mut string;
    dbg!(ref_mut_string);
    dbg!(ref_mut_string);
}
"#;

pub const MUT_MACRO_ERR_04 :&str = r#"pub fn main() {
    let mut string = format!("{}", "Hello");
    string.push('!');
    let ref_mut_string = &mut string;
    dbg!(ref_mut_string);
    println!("ref_mut_string = {}", ref_mut_string); // ref_mut_string is moved
}
"#;