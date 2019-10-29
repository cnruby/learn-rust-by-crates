pub const KW_FN_OK :&str = r#"// ok
fn main() {
    fn print_berry_names(berries: &Vec<&str>) {
        for berry in berries {
            println!("{}", berry);
        }
    }

    let berry_instances = vec!["Blackberry", "Strawberry"];
    print_berry_names(&berry_instances);

    dbg!(berry_instances);
}
"#;

pub const KW_FN_ERR :&str = r#"// error[E0384]
fn main() {
    fn print_berry_names(berries: Vec<&str>) {
        for berry in &berries {
            println!("{}", berry);
        }
    }

    let berry_instances = vec!["Blackberry", "Strawberry"];
    print_berry_names(berry_instances);

    dbg!(berry_instances);
}
"#;