#![allow(unused_variables)]

#[cfg(feature = "ok")]
fn main() {
    struct Struct(u8);

    let mut instance = Struct(42u8);
    //let new_instance = instance;
    // How to solve the problem?
    // Go to clone_struct.rs
    instance.0 = 33;
    println!("instance.data = {}", instance.0);
}

// error[E0382]
#[cfg(feature = "err")]
fn main() {
    struct Struct(u8);

    let mut instance = Struct(42u8);
    let new_instance = instance;
    instance.0 = 33;
    println!("instance.data = {}", instance.0);
}

#[cfg(all(not(feature = "ok"), not(feature = "err")))]
fn main() {
    use aide::hello;
    hello();
}

// https://doc.rust-lang.org/stable/error-index.html#E0382
