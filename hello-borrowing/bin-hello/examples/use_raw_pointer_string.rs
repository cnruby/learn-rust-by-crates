// File: ./examples/use_raw_pointer_string.rs
// clear && cargo run --example use_raw_pointer_string | bat -l cmd

#![allow(unused_variables)]

use std::slice;

fn main() {
    // ANCHOR: string-type-variable
    println!("");
    let instance: String = String::from("Hello");
    let ref_raw = instance.as_ptr();
    println!("instance value = {}", instance);
    println!("instance reference raw address = {:?}", ref_raw);
    // ANCHOR_END: string-type-variable

    // ANCHOR: string-ref-type-variable
    println!("");
    let ref_string: &String = &instance;
    let ref_string = &instance;
    println!("ref_string value = {}", ref_string);
    println!("ref_string owned address = {:p}", ref_string);
    let ref_raw_string: *const u8 = ref_string.as_ptr();
    // ANCHOR_END: string-ref-type-variable

    // ANCHOR: string-testing
    assert_eq!(ref_raw, ref_raw_string);
    // ANCHOR_END: string-testing

    println!("");
    let ref_slice = unsafe { slice::from_raw_parts(&ref_raw, 5) };
    dbg!(ref_slice);

    // ANCHOR: raw-testing
    assert_eq!(&ref_raw, &ref_slice[0]);
    // ANCHOR_END: raw-testing

    // ANCHOR: show-raw-u8
    println!("");
    let u8_slice = unsafe { slice::from_raw_parts(ref_raw, 5) };
    dbg!(u8_slice);
    // ANCHOR: show-raw-u8
}
