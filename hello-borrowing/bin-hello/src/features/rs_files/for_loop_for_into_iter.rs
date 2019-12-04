

pub const FOR_INTO_ITER_OK :&str = r#"pub fn main() {
    let mut instance = vec![33u8, 42];
    println!("1. instance = {:?}", instance);
    for item in instance.iter() {
        println!("item = {:p}", item);
    }
    println!("2. instance = {:?}", instance);
    let ref_instance = &mut instance;
    println!("1. ref_instance = {:p}", ref_instance);
    for item in ref_instance.into_iter() {
        *item = 3;
        println!("item = {:p}", item);
    }
    println!("3. instance = {:?}", instance);
    let ref_instance = &mut *instance;
    println!("1. ref_instance = {:p}", ref_instance);
    for item in ref_instance.into_iter() {
        *item = 4;
        println!("item = {:p}", item);
    }
    println!("4. instance = {:?}", instance);
    for item in instance.iter_mut() {
        *item = 5;
        println!("item = {:p}", item);
    }
    println!("5. instance = {:?}", instance);
    for item in instance.into_iter() {
        println!("item = {:p}", &item);
    }
}
"#;

pub const FOR_INTO_ITER_CP :&str = r#"pub fn main() {
    let mut instance = vec![33u8, 42];
    println!("1. instance = {:?}", instance);
    let ref_instance :&mut Vec<u8> = &mut instance;
    println!("1. ref_instance = {:?}", ref_instance);
    for item in ref_instance.into_iter() {
        *item = 1;
    }
    println!("2. ref_instance = {:?}", ref_instance);
    println!("2. instance = {:?}", instance);
}
"#;

pub const FOR_INTO_ITER_OKEY :&str = r#"pub fn main() {
    let mut instance = vec![33u8, 42];
    println!("1. instance = {:?}", instance);
    println!("1. &instance = {:p}", &instance);
    let ref_instance :&mut [u8] = &mut instance;
    println!("1. ref_instance = {:p}", ref_instance);
    for item in ref_instance.into_iter() {        
        *item = 1;
        println!("item = {:p}", item);
    }
    println!("2. ref_instance = {:p}", ref_instance);
    println!("2. instance = {:?}", instance);
}
"#;

pub const FOR_INTO_ITER_OKAY :&str = r#"pub fn main() {
    let instance = vec![33u8, 42];
    for item in instance.into_iter() {
        println!("item = {:p}", &item);
    }
}
"#;

pub const FOR_INTO_ITER_ERR_09 :&str = r#"pub fn main() {
    let instance = vec![33u8, 42];
    for item in instance.into_iter() {
        println!("item = {:p}", &item);
    }
    println!("instance = {:?}", instance);
}
"#;