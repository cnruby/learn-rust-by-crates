// File: ./bin-hello/examples/closure/mut_move.rs
// clear && cargo run --example closure --features ok -- mut_move | bat -l cmd
// clear && cargo run --example closure --features err_05
// clear && cargo run --example closure --features err_06

#[cfg(feature = "ok")]
pub fn adjoin() {
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





#[cfg(feature = "okay")]
pub fn adjoin() {
    let mut x = 0;
    let mut y = move |z: u32| {
        println!("1. inner x {}", x);
        println!("1. inner x {:p}", &x);

        x = x + z;
        //x += z;

        println!("2. inner x {}", x);
        println!("2. inner x {:p}", &x);
    };

    println!("1. outer x {}", x);
    println!("1. outer x {:p}", &x);
    y(2);
    println!("2. outer x {}", x);
    println!("2. outer x {:p}", &x);
}





#[cfg(feature = "err_05")]
pub fn adjoin() {
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





#[cfg(all(
    not(feature = "ok"),
    not(feature = "okay"),
    not(feature = "err_05"),
    //not(feature = "err_06"),
))]
pub fn adjoin() {
    use aide::*;
    hello();
}

// https://github.com/rust-lang/rust/issues/63220#issuecomment-558688508