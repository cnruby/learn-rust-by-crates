

pub const FOR_ARR_OK :&str = r#"pub fn main() {
    let instance = [1u8, 2, 3];
    println!("{:>17} {} {:p}", "instance ref", "=", &instance);
    print!("{:>17} {} ", "for ref", "=");
    for item in &instance {
        let ref_item: &u8 = item;
        print!("{:p} ", ref_item);
    }
    println!("");
    print!("{:>17} {} ", "for .iter()", "=");
    for item in instance.iter() {
        let ref_item: &u8 = item;
        print!("{:p} ", ref_item);
    }
    println!("");
    print!("{:>17} {} ", "for .into_iter()", "=");
    for item in instance.into_iter() {
        let ref_item: &u8 = item;
        print!("{:p} ", ref_item);
    }
    println!("");
    println!("{:>17} {} {:?}", "instance arr", "=", instance);
}
"#;

pub const FOR_ARR_ERR_01 :&str = r#"pub fn main() {
    let instance = [1u8, 2, 3];
    for item in instance {
        print!("{:p} ", item);
    }
    println!("instance array = {:?}", instance);
}
"#;