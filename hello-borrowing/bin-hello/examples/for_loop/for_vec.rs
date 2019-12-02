// File: ./bin-hello/examples/for_loop/for_vec.rs
// clear && cargo run --example for_loop --features ok -- for_vec | bat -l cmd
// clear && cargo run --example for_loop --features cp -- for_vec | bat -l cmd
// clear && cargo run --example for_loop --features err_02
// clear && cargo run --example for_loop -- for_vec

#[cfg(feature = "ok")]
pub fn adjoin() {
    // ANCHOR: feature-ok
    // File: ./bin-hello/examples/for_loop/for_vec.rs
    // #[cfg(feature = "ok")]

    let instance = vec![1u8, 2, 3];
    println!("{:>17} {} {:p}", "instance raw", "=", instance.as_ptr());

    print!("{:>17} {} ", "for ref", "=");
    for item in &instance {
        let ref_item: &u8 = item;
        print!("{:p} ", ref_item); // OK: item IS Pointer
    }
    println!("");

    println!("{:>17} {} {:?}", "instance vec", "=", instance);

    // ANCHOR_END: feature-ok
}

#[cfg(feature = "cp")]
// ANCHOR: feature-cp
pub fn adjoin() {
    // ANCHOR: feature-cp
    // File: ./bin-hello/examples/for_loop/for_vec.rs
    // #[cfg(feature = "cp")]

    let instance = vec![1, 2, 3];
    println!("{:>17} {} {:p}", "instance raw", "=", instance.as_ptr());

    print!("{:>17} {} ", "for u8", "=");
    for item in instance {
        let u8_item: u8 = item;
        print!("{:p} ", &u8_item);
    }
    println!("");

    // ANCHOR_END: feature-cp
}

#[cfg(feature = "err_02")]
// error[E0277]
pub fn adjoin() {
    // ANCHOR: feature-err
    // File: ./bin-hello/examples/for_loop/for_vec.rs
    // #[cfg(feature = "err_02")]

    let instance = vec![1, 2, 3];
    println!("{:>17} {} {:p}", "instance raw", "=", instance.as_ptr());

    print!("{:>17} {} ", "for u8", "=");
    for item in instance {
        let u8_item: u8 = item;
        print!("{:p} ", &u8_item);
    }
    println!("");

    println!("{:>17} {} {:?}", "instance vec", "=", instance); // error here

    // ANCHOR_END: feature-err
}

#[cfg(all(not(feature = "ok"), not(feature = "err_02"), not(feature = "cp")))]
pub fn adjoin() {
    use aide::hello;
    hello();
}
