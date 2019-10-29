pub const STACK_HEAD_OK :&str = r#"#![allow(unused)]

fn main() {
    let i = 42;
    let copy_i = i;
    println!("{}", i);
    println!("{}", copy_i);

    let v = vec![1, 2, 3, 4];
    let moved_v = v.clone();
    println!("{:?}", v);
    println!("{:?}", moved_v);

    #[derive(Debug, Clone)]
    struct Pair(u8);
    let pair = Pair(1);
    println!("original: {:?}", pair);
    let moved_pair = pair.clone();
    println!("copy: {:?}", moved_pair);
    println!("original: {:?}", pair);
}

"#;

pub const STACK_HEAD_ERR :&str = r#"#![allow(unused)]

fn main() {
    let i = 42;
    let copy_i = i;
    println!("{}", i);
    println!("{}", copy_i);

    let v = vec![1, 2, 3, 4];
    let moved_v = v; // value moved here
    println!("{:?}", v); // value borrowed here after move
    println!("{:?}", moved_v);

    #[derive(Debug, Clone)]
    struct Pair(u8);
    let pair = Pair(1);
    println!("original: {:?}", pair);
    let moved_pair = pair;
    println!("copy: {:?}", moved_pair);
    println!("original: {:?}", pair);
}
"#;
