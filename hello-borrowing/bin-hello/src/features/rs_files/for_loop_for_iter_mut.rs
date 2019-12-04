

pub const FOR_ITER_MUT_OK :&str = r#"pub fn main() {
    let mut instance = vec![33u8, 42];
    println!("1. instance = {:?}", instance);
    println!("1. &instance = {:p}", &instance);
    for item in instance.iter_mut() {
        *item = 1;
        println!("item = {:p}", item);
    }
    println!("2. instance = {:?}", instance);
    println!("２. &instance = {:p}", &instance);
}
"#;

pub const FOR_ITER_MUT_CP :&str = r#"pub fn main() {
    let mut instance = vec![33u8, 42];
    println!("1. instance = {:?}", instance);
    println!("1. &instance = {:p}", &instance);
    let ref_instance = &mut instance;
    println!("1. ref_instance = {:?}", ref_instance);
    println!("1. ref_instance = {:p}", ref_instance);
    for item in ref_instance.iter_mut() {
        *item = 1;
        println!("item = {:p}", item);
    }
    println!("2. ref_instance = {:?}", ref_instance);
    println!("2. ref_instance = {:p}", ref_instance);
    println!("2. instance = {:?}", instance);
}
"#;

pub const FOR_ITER_MUT_OKEY :&str = r#"pub fn main() {
    let mut instance = vec![33u8, 42];
    println!("1. instance = {:?}", instance);
    println!("1. &instance = {:p}", &instance);
    let ref_instance = &mut *instance;
    println!("1. ref_instance = {:p}", ref_instance);
    for item in ref_instance.iter_mut() {        
        *item = 1;
        println!("item = {:p}", item);
    }
    println!("2. ref_instance = {:p}", ref_instance);
    println!("2. instance = {:?}", instance);
}
"#;

pub const FOR_ITER_MUT_OKAY :&str = r#"pub fn main() {
    let mut instance = vec![33u8, 42];
    println!("1. instance = {:?}", instance);
    println!("1. &instance = {:p}", &instance);
    let ref_instance = &mut instance;
    println!("1. ref_instance = {:?}", ref_instance);
    println!("1. ref_instance = {:p}", ref_instance);
    for item in ref_instance.iter_mut() {
        *item = 1;
        println!("item = {:p}", item);
    }
    println!("2. ref_instance = {:?}", ref_instance);
    println!("2. ref_instance = {:p}", ref_instance);
    let ref_instance = &mut *instance;
    println!("3. ref_instance = {:p}", ref_instance);
    for item in ref_instance.iter_mut() {        
        *item = 1;
        println!("item = {:p}", item);
    }
    println!("3. ref_instance = {:?}", ref_instance);
    println!("3. ref_instance = {:p}", ref_instance);
    println!("2. instance = {:?}", instance);
}
"#;

pub const FOR_ITER_MUT_ERR_04 :&str = r#"pub fn main() {
    let mut instance = vec![33u8, 42];
    let ref_instance = &mut instance;
    for item in ref_instance {
        *item = 1;
    }
    println!("{:?}", ref_instance); // ERROR
    println!("{:?}", instance);
}
"#;

pub const FOR_ITER_MUT_ERR_05 :&str = r#"pub fn main() {
    let mut instance = vec![33u8, 42];
    let ref_instance = &mut *instance;
    for item in ref_instance {
        *item = 1;
    }
    
    println!("{:?}", ref_instance); //ERROR
    println!("{:?}", instance);
}
"#;

pub const FOR_ITER_MUT_ERR_06 :&str = r#"pub fn main() {
    let mut instance = vec![33u8, 42];
    println!("1. instance = {:?}", instance);
    for item in instance.into_iter() {
        item = 1;
    }
    println!("2. instance = {:?}", instance);
}
"#;