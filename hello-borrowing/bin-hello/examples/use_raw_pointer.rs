use std::slice;

fn main() {
    let instance = String::from("Hello");

    let ref_instance = &instance;
    println!("&instance = {:p}", ref_instance);

    let raw_pointer = instance.as_ptr();
    println!("raw_pointer = {:p}", raw_pointer);

    let slice = unsafe { slice::from_raw_parts(raw_pointer, 5) };
    dbg!(slice);

    let ref raw_pointer = instance.as_ptr();
    let slice = unsafe { slice::from_raw_parts(raw_pointer, 5) };
    dbg!(slice);

    //let instance = String::from("Hello");
    let v: Vec<char> = instance.chars().next().into_iter().collect();
    dbg!(v);

    for c in instance.chars() {
        println!("{:?}", c as u32);
    }
}
