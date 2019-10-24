#[cfg(feature = "ok")]
fn main() {
    let a: [u8; 0] = [];
    dbg!(a);
    let b = a.clone();
    let c = a;
    dbg!(a, b, c);
}

#[cfg(feature = "err")]
fn main() {
    let a: [u8; 0] = [];
    let b = a;
    //let c = a.clone();
    //println!("{:?} {:?} {:?}", a, b, c);
    println!("{:?} {:?}", a, b);
}

#[cfg(all(not(feature = "ok"), not(feature = "err")))]
fn main() {
    use aide::*;
    hello();
}
