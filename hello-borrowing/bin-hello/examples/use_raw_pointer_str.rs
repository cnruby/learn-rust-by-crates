// File: ./examples/use_raw_pointer_str.rs
// clear && cargo run --example use_raw_pointer_str | bat -l cmd

#![allow(unused_variables)]

use std::slice;

fn main() {
    println!("");
    let instance :String = String::from("Hello");
    let ref_raw :*const u8 = instance.as_ptr();
    println!("instance value = {}", instance);
    println!("instance reference raw address = {:?}", ref_raw);

    println!("");
    let ref_str :&str = &instance;
    let ref_str = instance.as_str();
    let ref_str :&str = instance.as_str();
    let ref_str :&str = &instance[0..=4];
    println!("ref_str value = {}", ref_str);
    println!("ref_str owned address = {:p}", ref_str);    
    let ref_str :*const u8 = ref_str.as_ptr();

    assert_eq!(ref_raw, ref_str);

    println!("");
    let slice = unsafe { slice::from_raw_parts(&ref_raw, 5) };
    dbg!(slice);

    assert_eq!(&ref_raw, &slice[0]);

    println!("");
    let slice = unsafe { slice::from_raw_parts(ref_raw, 5) };
    dbg!(slice);
}