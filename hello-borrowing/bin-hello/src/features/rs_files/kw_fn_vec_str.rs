

pub const VEC_STR_OK :&str = r#"pub fn main() {
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

pub const VEC_STR_ERR_02 :&str = r#"pub fn main() {
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