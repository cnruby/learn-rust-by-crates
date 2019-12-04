

pub const FOR_VEC_OK :&str = r#"pub fn main() {
    let instance = vec![1u8, 2, 3];
    println!("{:>17} {} {:p}", "instance raw", "=", instance.as_ptr());
    print!("{:>17} {} ", "for ref", "=");
    for item in &instance {
        let ref_item: &u8 = item;
        print!("{:p} ", ref_item); // OK: item IS Pointer
    }
    println!("");
    println!("{:>17} {} {:?}", "instance vec", "=", instance);
}
"#;

pub const FOR_VEC_CP :&str = r#"pub fn main() {
    let instance = vec![1, 2, 3];
    println!("{:>17} {} {:p}", "instance raw", "=", instance.as_ptr());
    print!("{:>17} {} ", "for u8", "=");
    for item in instance {
        let u8_item: u8 = item;
        print!("{:p} ", &u8_item);
    }
    println!("");
}
"#;

pub const FOR_VEC_ERR_02 :&str = r#"pub fn main() {
    let instance = vec![1, 2, 3];
    println!("{:>17} {} {:p}", "instance raw", "=", instance.as_ptr());
    print!("{:>17} {} ", "for u8", "=");
    for item in instance {
        let u8_item: u8 = item;
        print!("{:p} ", &u8_item);
    }
    println!("");
    println!("{:>17} {} {:?}", "instance vec", "=", instance); // error here
}
"#;