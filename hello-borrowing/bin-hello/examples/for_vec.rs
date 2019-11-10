// File: ./examples/for_vec.rs
// clear && cargo run --example for_vec --features ok | bat -l cmd
// clear && cargo run --example for_vec --features cp | bat -l cmd
// clear && cargo run --example for_vec --features err

#[cfg(feature = "ok")]
// ANCHOR: feature-ok
fn main() {
    let instance = vec![1u8, 2, 3];
    println!("{:>17} {} {:p}", "instance raw", "=", instance.as_ptr());

    print!("{:>17} {} ", "for ref", "=");
    for item in &instance {
        let ref_item :&u8 = item;
        print!("{:p} ", ref_item); // OK: item IS Pointer
    }
    println!("");

    println!("{:>17} {} {:?}", "instance vec", "=", instance);
}
// ANCHOR_END: feature-ok

#[cfg(feature = "cp")]
// ANCHOR: feature-cp
fn main() {
    let instance = vec![1, 2, 3];
    println!("{:>17} {} {:p}", "instance raw", "=", instance.as_ptr());

    print!("{:>17} {} ", "for u8", "=");
    for item in instance {
        let u8_item :u8 = item;
        print!("{:p} ", &u8_item);
    }
    println!("");
}
// ANCHOR_END: feature-cp

#[cfg(feature = "err")]
// error[E0277]
// ANCHOR: feature-err
fn main() {
    let instance = vec![1, 2, 3];
    println!("{:>17} {} {:p}", "instance raw", "=", instance.as_ptr());

    print!("{:>17} {} ", "for u8", "=");
    for item in instance {
        let u8_item :u8 = item;
        print!("{:p} ", &u8_item);
    }
    println!("");

    println!("{:>17} {} {:?}", "instance vec", "=", instance); // error here
}
// ANCHOR_END: feature-err

#[cfg(all(not(feature = "ok"), not(feature = "err"), not(feature = "cp") ))]
fn main() {
    use aide::hello;
    hello();
}
