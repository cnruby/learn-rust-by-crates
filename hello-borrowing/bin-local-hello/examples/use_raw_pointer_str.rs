// File: ./examples/use_raw_pointer_str.rs
// clear && cargo run --example use_raw_pointer_str | bat -l cmd

#![allow(unused_variables)]


use std::slice;
fn main() {
    // ANCHOR: string-type-variable
    println!("");
    let instance: String = String::from("Hello");
    let ref_raw: *const u8 = instance.as_ptr();
    println!("instance value = {}", instance);
    println!("instance reference raw address = {:?}", ref_raw);
    // ANCHOR_END: string-type-variable

    // ANCHOR: str-type-variable
    println!("");
    let ref_str: &str = &instance;
    let ref_str = instance.as_str();
    let ref_str: &str = instance.as_str();
    let ref_str: &str = &instance[0..=4];
    println!("ref_str value = {}", ref_str);
    println!("ref_str owned address = {:p}", ref_str);
    let ref_raw_str: *const u8 = ref_str.as_ptr();
    // ANCHOR_END: str-type-variable

    // ANCHOR: str-testing
    assert_eq!(ref_raw, ref_raw_str);
    // ANCHOR_END: str-testing

    // ANCHOR: show-raw-address
    println!("");
    let ref_slice: &[*const u8] = unsafe { slice::from_raw_parts(&ref_raw, 5) };
    dbg!(ref_slice);
    // ANCHOR_END: show-raw-address

    // ANCHOR: raw-testing
    assert_eq!(&ref_raw, &ref_slice[0]);
    // ANCHOR_END: raw-testing

    // ANCHOR: show-raw-u8
    println!("");
    let u8_slice: &[u8] = unsafe { slice::from_raw_parts(ref_raw, 5) };
    dbg!(u8_slice);
    // ANCHOR_END: show-raw-u8
}
