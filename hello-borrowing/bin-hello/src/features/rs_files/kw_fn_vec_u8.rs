

pub const VEC_U8_OK :&str = r#"pub fn main() {
    fn fn_borrow(vec_u8s: &Vec<u8>) {
        println!("Inside fn = {:p}", &vec_u8s);
    }
    let vec_instance: Vec<_> = vec![33, 42];
    println!("Before fn = {:p}", &vec_instance);
    fn_borrow(&vec_instance);
    println!("After fn = {:p}", &vec_instance);
    dbg!(vec_instance);
}
"#;

pub const VEC_U8_CP :&str = r#"pub fn main() {
    fn fn_borrow(vec_u8s: &Vec<u8>) {
        println!("Inside fn = {:p}", &vec_u8s);
    }
    let mut vec_instance: Vec<u8> = Vec::<u8>::new();
    vec_instance.push(33);
    vec_instance.push(42);
    println!("Before fn = {:p}", &vec_instance);
    fn_borrow(&vec_instance);
    println!("After fn = {:p}", &vec_instance);
    dbg!(vec_instance);
}
"#;

pub const VEC_U8_ERR_01 :&str = r#"pub fn main() {
    fn fn_borrow(vec_u8s: Vec<u8>) {
        println!("Inside fn = {:p}", &vec_u8s);
    }
    let vec_instance: Vec<_> = vec![33, 42];
    println!("Before fn = {:p}", &vec_instance);
    fn_borrow(vec_instance);
    println!("After fn = {:p}", &vec_instance);
    dbg!(vec_instance);
}
"#;