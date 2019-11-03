#[cfg(feature = "ok")]
fn main() {
    let instance = vec![1, 2, 3];
    println!("The variable instance before borrowing: {:?}", instance);

    // The variable instance begin to move here
    //let equal_to_val = move |z| {z == instance};
    let equal_to_val = move |z| z == instance;
    // The variable instance moved here

    // The variable instance borrowed here after move
    //println!("The variable instance after borrowing: {:?}", instance);

    // use the closure
    let y = vec![1, 2, 3];
    assert!(equal_to_val(y));
}

#[cfg(feature = "err")]
fn main() {
    let instance = vec![1, 2, 3];
    println!("The variable instance before borrowing: {:?}", instance);

    // The variable instance begin to move here
    //let equal_to_val = move |z| {z == instance};
    let equal_to_val = move |z| z == instance;
    // The variable instance moved here

    // The variable instance borrowed here after move
    println!("The variable instance after borrowing: {:?}", instance);

    // use the closure
    let y = vec![1, 2, 3];
    assert!(equal_to_val(y));
}

#[cfg(all(not(feature = "ok"), not(feature = "err")))]
fn main() {
    use aide::hello;
    hello();
}
