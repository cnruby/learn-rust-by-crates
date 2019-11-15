// File: ./bin-hello/examples/for_loop/for_arr/mod.rs
// clear && cargo run --example for_loop --features ok -- for_arr | bat -l cmd
// clear && cargo run --example for_loop --features err_01
// clear && cargo run --example for_loop -- for_arr

#[cfg(feature = "ok")]
pub fn adjoin() {
    // ANCHOR: feature-ok
    // File: ./bin-hello/examples/for_loop/for_arr/mod.rs
    // #[cfg(feature = "ok")]

    let instance = [1u8, 2, 3];
    println!("{:>17} {} {:p}", "instance ref", "=", &instance);

    print!("{:>17} {} ", "for ref", "=");
    for item in &instance {
        let ref_item: &u8 = item;
        print!("{:p} ", ref_item);
    }
    println!("");

    print!("{:>17} {} ", "for .iter()", "=");
    for item in instance.iter() {
        let ref_item: &u8 = item;
        print!("{:p} ", ref_item);
    }
    println!("");

    print!("{:>17} {} ", "for .into_iter()", "=");
    for item in instance.into_iter() {
        //let u8_item :u8 = item;  // ERROR: item IS &u8!!!
        let ref_item: &u8 = item;
        print!("{:p} ", ref_item);
    }
    println!("");

    println!("{:>17} {} {:?}", "instance arr", "=", instance);

    // ANCHOR_END: feature-ok
}

#[cfg(feature = "err_01")]
// error[E0277]
pub fn adjoin() {
    // ANCHOR: feature-err
    // File: ./bin-hello/examples/for_loop/for_arr/mod.rs
    // #[cfg(feature = "err_01")]

    let instance = [1u8, 2, 3];

    for item in instance {
        print!("{:p} ", item);
    }

    println!("instance array = {:?}", instance);

    // ANCHOR_END: feature-err
}

#[cfg(all(not(feature = "ok"), not(feature = "err_01")))]
pub fn adjoin() {
    use aide::hello;
    hello();
}
