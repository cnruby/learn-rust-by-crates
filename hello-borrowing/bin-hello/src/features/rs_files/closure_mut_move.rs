

pub const MUT_MOVE_OK :&str = r#"pub fn main() {
    let mut x = 0;
    let ref_x = &x;
    let y = |z: u32| -> u32 {
        println!("inner x = {}", *ref_x);
        println!("inner x = {:p}", ref_x);
        *ref_x + z
    };
    println!("1. outer x = {}", x);
    println!("1. outer x = {:p}", &x);
    x = y(2);
    println!("2. outer x = {}", x);
    println!("2. outer x = {:p}", &x);
}
"#;

pub const MUT_MOVE_OKAY :&str = r#"pub fn main() {
    let mut x = 0;
    let mut y = move |z: u32| {
        println!("1. inner x {}", x);
        println!("1. inner x {:p}", &x);
        x = x + z;
        println!("2. inner x {}", x);
        println!("2. inner x {:p}", &x);
    };
    println!("1. outer x {}", x);
    println!("1. outer x {:p}", &x);
    y(2);
    println!("2. outer x {}", x);
    println!("2. outer x {:p}", &x);
}
"#;

pub const MUT_MOVE_ERR_05 :&str = r#"pub fn main() {
    let mut x = 0;
    let mut y = |z: u32| {
        println!("1. inner x {}", x);
        println!("1. inner x {:p}", &x);
        x = x + z;
        println!("2. inner x {}", x);
        println!("2. inner x {:p}", &x);
    };
    println!("1. outer x {}", x);
    println!("1. outer x {:p}", &x);
    y(2);
    println!("2. outer x {}", x);
    println!("2. outer x {:p}", &x);
}
"#;