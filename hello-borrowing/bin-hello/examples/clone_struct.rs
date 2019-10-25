#[cfg(feature = "ok")]
fn main() {
    use std::ptr::eq;

    let a = String::new();
    let b = a.clone();
    //let c = a;
    // How to solve the problem?
    // Go to eq_ref.rs
    println!("{:?} {:?}", a, b);
    println!("{:p} {:p}", &a, &b);

    assert_eq!(a, b);
    assert!(!eq(&a, &b));
}

#[cfg(feature = "err")]
fn main() {
    use std::ptr::eq;

    let a = String::new();
    let b = a.clone();
    let c = a;
    println!("{:?} {:?}", a, b);
    println!("{:p} {:p}", &a, &b);

    assert_eq!(a, b);
    assert!(!eq(&a, &b));
}

#[cfg(all(not(feature = "ok"), not(feature = "err")))]
fn main() {
    use aide::*;
    hello();
}
