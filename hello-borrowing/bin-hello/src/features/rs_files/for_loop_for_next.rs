

pub const FOR_NEXT_OK :&str = r#"pub fn main() {
    let mut instance = vec![33u8; 5];
    let mut iter = instance.iter_mut();
    
    loop {
        match iter.next() {
            Some(x) => println!("{:?}", x),
            None => break,
        }
    }
    
    println!("{:?}", instance);
}
"#;

pub const FOR_NEXT_CP :&str = r#"pub fn main() {
    let mut instance = vec![33u8; 5];
    
    for index in 0..instance.len() {
        instance[index] = (index as u8) + instance[index];
        println!("{:?}", instance[index]);
    }
    println!("{:?}", instance);
}
"#;

pub const FOR_NEXT_OKEY :&str = r#"pub fn main() {
    let mut instance = vec![33u8; 5];
    
    for (index, item) in instance.iter_mut().enumerate() {
        *item = (index as u8) + *item;
        println!("{:?}", *item);
    }
    println!("{:?}", instance);
}
"#;

pub const FOR_NEXT_OKAY :&str = r#"pub fn main() {
    let instance: Vec<_> = vec![33u8; 5]
        .into_iter()
        .enumerate()
        .map(|(index, item)| (index as u8) + item)
        .collect();
    
    println!("{:?}", instance);
}
"#;

pub const FOR_NEXT_ERR_07 :&str = r#"pub fn main() {
    let mut instance = vec![33u8; 5];
    let iter = instance.iter_mut();
    
    loop {
        match iter.next() {
            Some(x) => println!("{:?}", x),
            None => break,
        }
    }
    
    println!("{:?}", instance);
}
"#;

pub const FOR_NEXT_ERR_08 :&str = r#"pub fn main() {
    let instance = vec![33u8; 5];
    let mut iter = instance.into_iter();
    
    loop {
        match iter.next() {
            Some(x) => println!("{:?}", x),
            None => break,
        }
    }
    
    println!("{:?}", instance);
    
}
"#;