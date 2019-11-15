// File: ./examples/for_vec_iter.rs
// clear && cargo run --example for_vec_iter --features ok | bat -l rs
// clear && cargo run --example for_vec_iter --features cp
// clear && cargo run --example for_vec_iter --features err

#[cfg(feature = "ok")]
// ANCHOR: feature-ok
fn main() {
    let instance = vec![1u8, 2, 3];
    println!("{:>13} {} {:p}", "instance raw", "=", instance.as_ptr());

    print!("{:>13} {} ", "for vec ref", "=");
    for item in &instance {
        let ref_item :&u8 = item;
        print!("{:p} ", ref_item); // OK: item IS Pointer
    }    
    println!("");

    print!("{:>13} {} ", "for vec iter", "=");
    for item in instance.iter() {
        let ref_item :&u8 = item;
        print!("{:p} ", ref_item);  // OK: item IS Pointer
    }
    println!("");

    println!("{:>13} {} {:?}", "instance vec", "=", instance);
}
// ANCHOR_END: feature-ok

#[cfg(feature = "cp")]
// ANCHOR: feature-cp
fn main() {
    let instance = vec![1u8, 2, 3];
    println!("{:>13} {} {:p}", "instance raw", "=", instance.as_ptr());

    print!("{:>13} {} ", "for into_iter", "=");
    for item in instance.into_iter() {
        let u8_item :u8 = item;
        //print!("{:p} ", item); // ERROR: item IS {integer}, NOT Pointer
        print!("{:p} ", &u8_item);
    }
    println!("");

    //println!("{:>13} {} {:?}", "instance vec", "=", instance);
}
// ANCHOR_END: feature-cp

#[cfg(feature = "err")]
// error[E0277]
// ANCHOR: feature-err
fn main() {
    let instance = vec![1, 2, 3];
    println!("{:>13} {} {:p}", "instance raw", "=", instance.as_ptr());

    print!("{:>13} {} ", "for into_iter", "=");
    for item in instance.into_iter() {
        let u8_item :u8 = item;
        print!("{:p} ", &u8_item);
    }
    println!("");

    println!("{:>13} {} {:?}", "instance vec", "=", instance);
}
// ANCHOR_END: feature-err

#[cfg(all(not(feature = "ok"), not(feature = "cp"), not(feature = "err") ))]
fn main() {
    use aide::hello;
    hello();
}
