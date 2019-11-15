fn main() {
    let immut_string = String::from("Hello");
    dbg!(immut_string);

    let mut mut_string = String::new();
    mut_string.push_str("Hello");
    dbg!(mut_string);
}
