

pub const FOR_VEC_ITER_OK :&str = r#"pub fn main() {
    let instance = vec![1u8, 2, 3];
    println!("{:>13} {} {:p}", "instance raw", "=", instance.as_ptr());
    print!("{:>13} {} ", "for vec ref", "=");
    for item in &instance {
        let ref_item: &u8 = item;
        print!("{:p} ", ref_item); // OK: item IS Pointer
    }
    println!("");
    print!("{:>13} {} ", "for vec iter", "=");
    for item in instance.iter() {
        let ref_item: &u8 = item;
        print!("{:p} ", ref_item); // OK: item IS Pointer
    }
    println!("");
    println!("{:>13} {} {:?}", "instance vec", "=", instance);
}
"#;

pub const FOR_VEC_ITER_CP :&str = r#"pub fn main() {
    let instance = vec![1u8, 2, 3];
    println!("{:>13} {} {:p}", "instance raw", "=", instance.as_ptr());
    print!("{:>13} {} ", "for into_iter", "=");
    for item in instance.into_iter() {
        let u8_item: u8 = item;
        print!("{:p} ", &u8_item);
    }
    println!("");
}
"#;

pub const FOR_VEC_ITER_ERR_03 :&str = r#"pub fn main() {
    let instance = vec![1, 2, 3];
    println!("{:>13} {} {:p}", "instance raw", "=", instance.as_ptr());
    print!("{:>13} {} ", "for into_iter", "=");
    for item in instance.into_iter() {
        let u8_item: u8 = item;
        print!("{:p} ", &u8_item);
    }
    println!("");
    println!("{:>13} {} {:?}", "instance vec", "=", instance);
}
"#;