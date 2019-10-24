#[cfg(feature = "ok")]
fn main() {
    let a = [1, 2, 3];
    for i in a.iter() {
        // for i in &a {
        print!("{} ", i);
    }
    println!("\nv = {:?}", a);

    let v = vec![1, 2, 3];
    for i in &v {
        // for i in v.iter() {
        print!("{} ", i);
    }
    println!("\nv = {:?}", v);
}

// error[E0277]
#[cfg(feature = "err")]
fn main() {
    let a = [1, 2, 3];
    for i in a {
        print!("{} ", i);
    }
    println!("\na = {:?}", a);

    let v = vec![1, 2, 3];
    for i in v {
        print!("{} ", i);
    }
    println!("v = {}", v); // error here
}

#[cfg(all(not(feature = "ok"), not(feature = "err")))]
fn main() {
    use aide::hello;
    hello();
}
