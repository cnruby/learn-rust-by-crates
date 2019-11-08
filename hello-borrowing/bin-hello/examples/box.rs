use owning_ref::BoxRef;
use std::slice;

fn main() {
    let instance = String::from("Hello");

    let ref_raw = instance.as_str();
    println!("ref_raw = {:p}", ref_raw);
    let raw_pointer = instance.as_ptr();
    let slice = unsafe { slice::from_raw_parts(raw_pointer, 5) };
    dbg!(slice);
    println!("raw_pointer = {:p}", raw_pointer);

    let ref raw_pointer = instance.as_ptr();
    let slice = unsafe { slice::from_raw_parts(raw_pointer, 5) };
    dbg!(slice);

    let ref_instance = &instance;
    println!("&instance = {:p}", ref_instance);

    //let instance = String::from("Hello");
    let v: Vec<char> = instance.chars().next().into_iter().collect();
    dbg!(v);

    for c in instance.chars() {
        println!("{:?}", c as u32);
    }

    //
    let instance = String::from("  Hello  ");
    let line: Box<str> = instance.into_boxed_str();
    let ref raw_pointer = line.as_ptr();
    let slice = unsafe { slice::from_raw_parts(raw_pointer, 9) };
    dbg!(slice);

    let trimmed: Box<str> = line.trim().into();
    let ref raw_pointer = trimmed.as_ptr();
    //let raw_pointer = instance.as_ptr();
    let slice = unsafe { slice::from_raw_parts(raw_pointer, 5) };
    dbg!(slice);

    let instance = String::from("  Hello  ");
    let line: Box<str> = instance.into_boxed_str();
    let ref raw_pointer = line.as_ptr();
    //let raw_pointer = line.as_ptr();
    let slice = unsafe { slice::from_raw_parts(raw_pointer, 9) };
    dbg!(slice);

    let trimmed = line.trim();
    let ref raw_pointer = trimmed.as_ptr();
    //let raw_pointer = trimmed.as_ptr();
    let slice = unsafe { slice::from_raw_parts(raw_pointer, 5) };
    dbg!(slice);

    let instance = String::from("  Hello  ");
    let line: BoxRef<str> = instance.into_boxed_str().into();
    let ref raw_pointer = line.as_ptr();
    //let raw_pointer = line.as_ptr();
    let slice = unsafe { slice::from_raw_parts(raw_pointer, 9) };
    dbg!(slice);

    let trimmed: BoxRef<str> = line.map(|s| s.trim());
    let ref raw_pointer = trimmed.as_ptr();
    //let raw_pointer = trimmed.as_ptr();
    let slice = unsafe { slice::from_raw_parts(raw_pointer, 5) };
    dbg!(slice);
}
