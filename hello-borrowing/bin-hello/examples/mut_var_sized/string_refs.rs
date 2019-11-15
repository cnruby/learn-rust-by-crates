// File: ./examples/mut_var_sized/string_refs.rs
// clear && cargo run --example mut_var_sized --features ok -- string_refs | bat -l rs
// clear && cargo run --example mut_var_sized --features err_04
// clear && cargo run --example mut_var_sized --features err_05
// clear && cargo run --example mut_var_sized --features err_06

#![allow(unused_variables)]
#![allow(unused_mut)]

#[cfg(feature = "ok")]
pub fn adjoin() {
    // ANCHOR: feature-ok
    // File: ./examples/mut_var_sized/string_refs.rs
    // #[cfg(feature = "ok")]

    let mut instance = String::new();
    instance.push_str("hello");

    let ref_mut_instance :&mut String = &mut instance;
    ref_mut_instance.push_str(" world");

    let ref_instance :&String = ref_mut_instance;
    println!("ref_instance = {}", ref_instance);
    println!("ref_mut_instance = {}", ref_mut_instance);

    ref_mut_instance.make_ascii_uppercase(); // ref_instance is moved after here
    //println!("ref_instance = {}", ref_instance);
    println!("ref_mut_instance = {}", ref_mut_instance);

    instance.push('!'); // ref_mut_instance is moved after here
    //println!("ref_instance = {}", ref_mut_instance);

    println!("instance = {}", instance);

    // ANCHOR_END: feature-ok
}



#[cfg(feature = "err_04")]
pub fn adjoin() {
    // ANCHOR: feature-error_01
    // File: ./examples/mut_var_sized/string_refs.rs
    // ANCHOR = "string_refs-error_01"
    // error[E0502]: cannot borrow `*ref_mut_instance` as mutable because it is also borrowed as immutable

    let mut instance = String::new();
    instance.push_str("hello");

    let ref_mut_instance :&mut String = &mut instance;
    ref_mut_instance.push_str(" world");

    let ref_instance :&String = ref_mut_instance;
    println!("ref_instance = {}", ref_instance);
    println!("ref_mut_instance = {}", ref_mut_instance);

    ref_mut_instance.make_ascii_uppercase(); // ref_instance is moved after here
    println!("ref_instance = {}", ref_instance); // ERROR
    println!("ref_mut_instance = {}", ref_mut_instance);

    instance.push('!'); // ref_mut_instance is moved after here
    //println!("ref_instance = {}", ref_mut_instance);

    println!("instance = {}", instance);

    // ANCHOR_END: feature-error_01
}



#[cfg(feature = "err_05")]
pub fn adjoin() {
    // ANCHOR: feature-error_02
    // File: ./examples/mut_var_sized/string_refs.rs
    // ANCHOR = "string_refs-error_02"
    // error[E0499]: cannot borrow `instance` as mutable more than once at a time
    
    let mut instance = String::new();
    instance.push_str("hello");

    let ref_mut_instance :&mut String = &mut instance;
    ref_mut_instance.push_str(" world");

    let ref_instance :&String = ref_mut_instance;
    println!("ref_instance = {}", ref_instance);
    println!("ref_mut_instance = {}", ref_mut_instance);

    ref_mut_instance.make_ascii_uppercase(); // ref_instance is moved after here
    //println!("ref_instance = {}", ref_instance);
    println!("ref_mut_instance = {}", ref_mut_instance);

    instance.push('!'); // ref_mut_instance is moved after here
    println!("ref_instance = {}", ref_mut_instance);

    println!("instance = {}", instance);

    // ANCHOR_END: feature-error_02
}




#[cfg(feature = "err_06")]
pub fn adjoin() {
    // ANCHOR: feature-error_03
    // File: ./examples/mut_var_sized/string_refs.rs
    // ANCHOR = "string_refs-error_03"
    // error[E0502]: cannot borrow `instance` as immutable because it is also borrowed as mutable

    let mut instance = String::new();
    instance.push_str("hello");

    let ref_mut_instance :&mut String = &mut instance;
    ref_mut_instance.push_str(" world");

    //let ref_instance :&String = ref_mut_instance;
    let ref_instance :&String = &instance;
    println!("ref_instance = {}", ref_instance);
    println!("ref_mut_instance = {}", ref_mut_instance);  // ERROR

    ref_mut_instance.make_ascii_uppercase(); // ref_instance is moved after here
    //println!("ref_instance = {}", ref_instance);
    println!("ref_mut_instance = {}", ref_mut_instance);

    instance.push('!'); // ref_mut_instance is moved after here
    //println!("ref_instance = {}", ref_mut_instance);

    println!("instance = {}", instance);

    // ANCHOR_END: feature-error_03
}



#[cfg(all(
    not(feature = "ok"),
    not(feature = "err_04"),
    not(feature = "err_05"),
    not(feature = "err_06"),
))]
pub fn adjoin() {
    use aide::hello;
    hello();
}
