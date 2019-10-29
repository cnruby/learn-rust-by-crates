pub const KW_LET_MUT_OK :&str = r#"#![allow(unused_variables)]
#![allow(unused_assignments)]

fn main() {
    let mut instance = 42_u8;

    instance = 33;
}
"#;

pub const KW_LET_MUT_ERR :&str = r#"#![allow(unused_variables)]
#![allow(unused_assignments)]

fn main() {
    let instance = 42_u8;

    instance = 33;
}
"#;
