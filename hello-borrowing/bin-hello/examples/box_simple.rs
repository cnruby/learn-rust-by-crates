fn main() {
    let x = String::from("Hello");
    let y = &x;
    dbg!(y.len());
    dbg!(x.as_ptr());
    dbg!(y.as_ptr());
    println!("{:p}", x.as_ptr());
    println!("{:p}", y);

    let x = 42u8;
    let y = &x;
    //dbg!(y.len());
    //dbg!(x.as_ptr());
    //dbg!(y.as_ptr());
    //println!("{:p}", x.as_ptr());
    println!("{:p}", y);

    let x = Box::new("Hello");
    let y = &x;
    dbg!(x.as_ptr());
    dbg!(x.as_ref());
    dbg!(y.len());
    println!("{:p}", x);
    println!("{:p}", x.as_ref());
    println!("{}", x);
    println!("{}", x.as_ref());
}
