// File: ./examples/use_raw_pointer_string.rs
// clear && cargo run --example use_raw_pointer_string | bat -l cmd

#![allow(unused_variables)]

use std::slice;

fn main() {
    println!("");
    let instance :String = String::from("Hello");
    let ref_raw = instance.as_ptr();
    println!("instance value = {}", instance);
    println!("instance reference raw address = {:?}", ref_raw);

    println!("");
    let ref_string :&String = &instance;
    //let ref_string = &instance;
    println!("ref_string value = {}", ref_string);
    println!("ref_string owned address = {:p}", ref_string);
    let ref_string :*const u8 = ref_string.as_ptr();

    assert_eq!(ref_raw, ref_string);

    println!("");
    let slice = unsafe { slice::from_raw_parts(&ref_raw, 5) };
    dbg!(slice);
    
    assert_eq!(&ref_raw, &slice[0]);

    println!("");
    let slice = unsafe { slice::from_raw_parts(ref_raw, 5) };
    dbg!(slice);
}
