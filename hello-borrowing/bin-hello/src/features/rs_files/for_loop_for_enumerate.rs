

pub const FOR_ENUMERATE_OK :&str = r#"pub fn main() {
    let instance: Vec<_> = vec![33u8; 5]
        .into_iter()
        .enumerate()
        .map(|(index, item)| {
            let ret = (index as u8) + item;            
            println!("{:?}", ret);
            ret
        })
        .collect();
    
    println!("{:?}", instance);
}
"#;

pub const FOR_ENUMERATE_CP :&str = r#"pub fn main() {
    let mut instance = vec![33u8; 5];
    
    for index in 0..instance.len() {
        instance[index] = (index as u8) + instance[index];
        println!("{:?}", instance[index]);
    }
    println!("{:?}", instance);
}
"#;

pub const FOR_ENUMERATE_OKEY :&str = r#"pub fn main() {
    let mut instance = vec![33u8; 5];
    
    for (index, item) in instance.iter_mut().enumerate() {
        *item = (index as u8) + *item;
        println!("{:?}", *item);
    }
    println!("{:?}", instance);
}
"#;

pub const FOR_ENUMERATE_OKAY :&str = r#"pub fn main() {
    let instance: Vec<_> = vec![33u8; 5]
        .into_iter()
        .enumerate()
        .map(|(index, item)| (index as u8) + item)
        .collect();
    
    println!("{:?}", instance);
}
"#;