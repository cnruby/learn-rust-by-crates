pub const KW_MUT_OK :&str = r#"// ok
fn main() {
    let berry_instances = vec!["Blackberry", "Strawberry"];

    let ref first_share_ref = &berry_instances; // immutable reference
    let ref second_share_ref = &berry_instances; // immutable reference

    println!("{:?} {:?}", first_share_ref, second_share_ref);
}
"#;

pub const KW_MUT_ERR :&str = r#"// error[E0502]
fn main() {
    let mut berry_instances = vec!["Blackberry", "Strawberry"];

    let ref first_share_immut = &berry_instances; // immutable reference
    let ref second_share_mut = &mut berry_instances; // mutable reference

    println!("{:?} {:?}", first_share_immut, second_share_mut);
}
"#;