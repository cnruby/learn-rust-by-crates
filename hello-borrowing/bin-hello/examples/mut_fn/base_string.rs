// File: ./examples/mut_fn/base_string.rs
// clear && cargo run --example mut_fn --features okey -- base_string | bat -l rs
// clear && cargo run --example mut_fn --features err_01
// clear && cargo run --example mut_fn --features err_02

#![allow(unused_variables)]

#[cfg(feature = "ok")]
pub fn adjoin() {
    // ANCHOR: feature-ok
    // File: ./examples/mut_fn/base_string.rs
    // #[cfg(feature = "ok")]
    // FIXED

    fn one(input: &mut String) {
        input.push_str(", world");
        println!("one() input = {:p}", input);
    }

    fn two(input: &mut String) {
        input.push('!');
        println!("two() input = {:p}", input);
    }
 
    let mut instance = String::new();
    println!("instance = {:p}", &instance);

    instance.push_str("Hello");
    let mut_ref = &mut instance;
    println!("mut_ref = {:p}", mut_ref);

    one(mut_ref);
    two(mut_ref);
    
    println!("instance = {:p}", &instance);
    println!("{}", instance);

    // ANCHOR_END: feature-ok
}




#[cfg(feature = "cp")]
pub fn adjoin() {
    // ANCHOR: feature-cp
    // File: ./examples/mut_fn/base_string.rs
    // #[cfg(feature = "cp")]
    // 


    fn one(input: &str) -> String {        
        println!("input = {:p}", input);
        let result = format!("{}{}", input, "!");
        println!("input = {:p}", input);
        println!("input = {}", input);
        println!("result = {:p}", &result);
        println!("result = {}", result);
        result
    }

    fn two(input: &str) -> String {
        println!("input = {:p}", input);
        let result = format!("{}{}", input, "?");
        println!("input = {:p}", input);
        println!("input = {}", input);
        println!("result = {:p}", &result);
        println!("result = {}", result);
        result
    }
 
    let mut instance = String::new();
    println!("instance = {:p}", &instance);

    instance.push_str("Hello");

    let mut mut_ref :&str = &instance;
    println!();
    println!("mut_ref = {:p}", mut_ref);
    instance = one(mut_ref);    
    
    mut_ref = &instance;
    println!();
    println!("mut_ref = {:p}", mut_ref);
    instance = two(mut_ref);

    println!();
    println!("instance = {:p}", &instance);
    println!("{}", instance);


    // ANCHOR_END: feature-cp
}





#[cfg(feature = "okay")]
pub fn adjoin() {
    // ANCHOR: feature-okay
    // File: ./examples/mut_fn/base_string.rs
    // #[cfg(feature = "okay")]
    // FIXED

    fn one(mut input: String) {
        input.push_str(", world");        
        println!("input = {:p}", &input);
        println!("input = {}", input);
    }
 
    let mut instance = String::new();
    println!("instance = {:p}", &instance);
    instance.push_str("Hello");

    one(instance);

    // ANCHOR_END: feature-okay
}




#[cfg(feature = "okey")]
pub fn adjoin() {
    // ANCHOR: feature-okey
    // File: ./examples/mut_fn/base_string.rs
    // #[cfg(feature = "okey")]
    // 

    fn one(x: &str, y: &String) {
        x.to_ascii_uppercase();
    }

    let first_str = "Hello";
    let second_str = "World".to_string();
    one(first_str, &second_str);
    
    // ANCHOR_END: feature-okey
}




#[cfg(feature = "err_04")]
pub fn adjoin() {
    // ANCHOR: feature-error_01
    // File: ./examples/mut_fn/base_string.rs
    // #[cfg(feature = "err_04")]
    // error[E0382]: borrow of moved value: `instance`
    // FIXED

    fn one(mut input: String) {
        input.push_str(", world");
        println!("input = {:p}", &input);
    }

    let mut instance = String::new();
    println!("instance = {:p}", &instance);
    instance.push_str("Hello");

    one(instance);
    
    println!("{}", instance);

    // ANCHOR_END: feature-error_01
}




#[cfg(feature = "err_05")]
pub fn adjoin() {
    // ANCHOR: feature-error_02
    // File: ./examples/mut_fn/base_string.rs
    // #[cfg(feature = "err_05")]
    // error[E0596]: cannot borrow `*input` as mutable, as it is behind a `&` reference
    // FIXED

    // fn one(input: &mut String) {  //OK
    fn one(input: &String) {
        input.push_str(", world");
        println!("input = {:p}", input);
    }
    
    // fn two(input: &mut String) {  //OK
    fn two(input: &String) {
        input.push('!');
        println!("input = {:p}", input);
    }
    
    let mut instance = String::new();
    println!("instance = {:p}", &instance);
    
    instance.push_str("Hello");
    let mut_ref = &mut instance;
    println!("mut_ref = {:p}", mut_ref);
    
    one(mut_ref);
    two(mut_ref);
    
    println!("instance = {:p}", &instance);
    println!("{}", instance);
    
    // println!("instance = {:p}", &instance);
    //println!("{}", instance);    


    // ANCHOR_END: feature-error_02
}




#[cfg(feature = "err_06")]
pub fn adjoin() {
    // ANCHOR: feature-error_03
    // File: ./examples/mut_fn/base_string.rs
    // #[cfg(feature = "err_06")]
    // error[E0596]: cannot borrow `input` as mutable, as it is not declared as mutable
    // FIXED

    fn one(input: String) {
        input.push_str(", world");
        println!("input = {:p}", &input);
    }

    let mut instance = String::new();
    println!("instance = {:p}", &instance);
    instance.push_str("Hello");

    one(instance);

    // ANCHOR_END: feature-error_03
}





#[cfg(all(
    not(feature = "ok"),
    not(feature = "cp"),
    not(feature = "okay"),
    not(feature = "okey"),
    not(feature = "err_04"),
    not(feature = "err_05"),
    not(feature = "err_06"),
))]
pub fn adjoin() {
    use aide::*;
    hello();
}

// https://play.rust-lang.org/?gist=cae31659a0caa396c2702039b5e16964&version=stable