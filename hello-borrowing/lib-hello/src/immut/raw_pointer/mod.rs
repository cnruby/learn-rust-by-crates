// Goto the Project "bin-local-hello"
// cargo run --example usage -- raw_pointer_str
// cargo run --example usage -- raw_pointer_string

#![allow(unused_variables)]

pub fn use_raw_pointer_str() {
    // ANCHOR: use_raw_pointer_str
    // File: lib-hello/src/immut/raw_pointer/mod.rs
    // Function: use_raw_pointer_str()

    use std::slice;
    // ANCHOR: string-type-variable
    println!();
    let instance: String = String::from("Hello");
    let ref_raw: *const u8 = instance.as_ptr();
    println!("instance value = {}", instance);
    println!("instance reference raw address = {:?}", ref_raw);
    // ANCHOR_END: string-type-variable

    // ANCHOR: str-type-variable
    println!();
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
    println!();
    let ref_slice: &[*const u8] = unsafe { slice::from_raw_parts(&ref_raw, 5) };
    dbg!(ref_slice);
    // ANCHOR_END: show-raw-address

    // ANCHOR: raw-testing
    assert_eq!(&ref_raw, &ref_slice[0]);
    // ANCHOR_END: raw-testing

    // ANCHOR: show-raw-u8
    println!();
    let u8_slice: &[u8] = unsafe { slice::from_raw_parts(ref_raw, 5) };
    dbg!(u8_slice);
    // ANCHOR_END: show-raw-u8

    // ANCHOR_END: use_raw_pointer_str
}



pub fn use_raw_pointer_string() {
    // ANCHOR: use_raw_pointer_string
    // File: lib-hello/src/immut/raw_pointer/mod.rs
    // Function: use_raw_pointer_string()
    
    use std::slice;
    // ANCHOR: string-type-variable
    println!();
    let instance: String = String::from("Hello");
    let ref_raw = instance.as_ptr();
    println!("instance value = {}", instance);
    println!("instance reference raw address = {:?}", ref_raw);
    // ANCHOR_END: string-type-variable

    // ANCHOR: string-ref-type-variable
    println!();
    let ref_string: &String = &instance;
    let ref_string = &instance;
    println!("ref_string value = {}", ref_string);
    println!("ref_string owned address = {:p}", ref_string);
    let ref_raw_string: *const u8 = ref_string.as_ptr();
    // ANCHOR_END: string-ref-type-variable

    // ANCHOR: string-testing
    assert_eq!(ref_raw, ref_raw_string);
    // ANCHOR_END: string-testing

    println!();
    let ref_slice = unsafe { slice::from_raw_parts(&ref_raw, 5) };
    dbg!(ref_slice);

    // ANCHOR: string-raw-testing
    assert_eq!(&ref_raw, &ref_slice[0]);
    // ANCHOR_END: string-raw-testing

    // ANCHOR: show-string-raw-u8
    println!();
    let u8_slice = unsafe { slice::from_raw_parts(ref_raw, 5) };
    dbg!(u8_slice);
    // ANCHOR: show-string-raw-u8

    // ANCHOR_END: use_raw_pointer_string
}



pub fn use_raw_pointer() {
    // ANCHOR: use_raw_pointer
    // File: lib-hello/src/immut/raw_pointer/mod.rs
    // Function: use_raw_pointer()
    
    use std::slice;
    let instance = String::from("Hello");

    let ref_instance = &instance;
    println!("&instance = {:p}", ref_instance);

    let raw_pointer = instance.as_ptr();
    println!("raw_pointer = {:p}", raw_pointer);

    let slice = unsafe { slice::from_raw_parts(raw_pointer, 5) };
    dbg!(slice);

    let raw_pointer = &instance.as_ptr();
    let slice = unsafe { slice::from_raw_parts(raw_pointer, 5) };
    dbg!(slice);

    //let instance = String::from("Hello");
    let v: Vec<char> = instance.chars().next().into_iter().collect();
    dbg!(v);

    for c in instance.chars() {
        println!("{:?}", c as u32);
    }

    // ANCHOR_END: use_raw_pointer
}
