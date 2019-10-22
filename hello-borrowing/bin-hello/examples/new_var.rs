#![allow(unused)]
struct Struct(u8);

// Run OK:
// cargo run --bin bw -- --file new_var --mode ok
// target/debug/bw --file new_var --mode ok
// cargo install borrowing_exerci
// bw --file new_var --mode ok

// Compile-Time Error:
// cargo run --bin bw -- -f new_var -m error
// cargo run --bin bw -- -f new_var
// target/debug/bw -f new_var -m error
// target/debug/bw -f new_var
// cargo install borrowing_exerci
// bw --file new_var -m error
// bw -f new_var

#[cfg(feature = "ok")]
fn main() {
    let mut instance = Struct(42u8);
    // let new_instance = instance;
    instance.0 = 33;
    println!("instance.data = {}", instance.0);
}

// error[E0382]
#[cfg(feature = "error")]
fn main() {
    let mut instance = Struct(42u8);
    let new_instance = instance;
    instance.0 = 33;
    println!("instance.data = {}", instance.0);
}

#[cfg(not(feature = "ok"))]
#[cfg(not(feature = "error"))]
fn main() {
    use aide::hello;
    hello();
}
// https://doc.rust-lang.org/stable/error-index.html#E0382
