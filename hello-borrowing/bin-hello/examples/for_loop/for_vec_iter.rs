// File: ./bin-hello/examples/for_loop/for_vec_iter.rs
// clear && cargo run --example for_loop --features ok -- for_vec_iter | bat -l cmd
// clear && cargo run --example for_loop --features cp -- for_vec_iter | bat -l cmd
// clear && cargo run --example for_loop --features err_03
// clear && cargo run --example for_loop -- for_vec_iter

//=======


//=======
#[cfg(feature = "ok")]
pub fn adjoin() {
    // ANCHOR: feature-ok
    // File: ./bin-hello/examples/for_loop/for_vec_iter.rs
    // #[cfg(feature = "ok")]

    let instance = vec![1u8, 2, 3];
    println!("{:>13} {} {:p}", "instance raw", "=", instance.as_ptr());

    print!("{:>13} {} ", "for vec ref", "=");
    for item in &instance {
        let ref_item: &u8 = item;
        print!("{:p} ", ref_item); // OK: item IS Pointer
    }
    println!("");

    print!("{:>13} {} ", "for vec iter", "=");
    for item in instance.iter() {
        let ref_item: &u8 = item;
        print!("{:p} ", ref_item); // OK: item IS Pointer
    }
    println!("");

    println!("{:>13} {} {:?}", "instance vec", "=", instance);

    // ANCHOR_END: feature-ok
}



//=======
#[cfg(feature = "cp")]
pub fn adjoin() {
    // ANCHOR: feature-cp
    // File: ./bin-hello/examples/for_loop/for_vec_iter.rs
    // #[cfg(feature = "cp")]

    let instance = vec![1u8, 2, 3];
    println!("{:>13} {} {:p}", "instance raw", "=", instance.as_ptr());

    print!("{:>13} {} ", "for into_iter", "=");
    for item in instance.into_iter() {
        let u8_item: u8 = item;
        //print!("{:p} ", item); // ERROR: item IS {integer}, NOT Pointer
        print!("{:p} ", &u8_item);
    }
    println!("");

    //println!("{:>13} {} {:?}", "instance vec", "=", instance);

    // ANCHOR_END: feature-cp
}




//=======
#[cfg(feature = "err_03")]
// error[E0277]
pub fn adjoin() {
    // ANCHOR: feature-err
    // File: ./bin-hello/examples/for_loop/for_vec_iter.rs
    // #[cfg(feature = "err_03")]

    let instance = vec![1, 2, 3];
    println!("{:>13} {} {:p}", "instance raw", "=", instance.as_ptr());

    print!("{:>13} {} ", "for into_iter", "=");
    for item in instance.into_iter() {
        let u8_item: u8 = item;
        print!("{:p} ", &u8_item);
    }
    println!("");

    println!("{:>13} {} {:?}", "instance vec", "=", instance);

    // ANCHOR_END: feature-err
}



//=======
#[cfg(all(not(feature = "ok"), not(feature = "cp"), not(feature = "err_03")))]
pub fn adjoin() {
    use aide::hello;
    hello();
}
