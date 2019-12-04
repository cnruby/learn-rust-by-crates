// File: ./examples/mut_fn/base_str.rs
// clear && cargo run --example mut_fn --features okey -- base_str | bat -l rs
// clear && cargo run --example mut_fn --features err_01
// clear && cargo run --example mut_fn --features err_02

//=======
#![allow(unused_variables)]



//=======
#[cfg(feature = "ok")]
pub fn adjoin() {
    // ANCHOR: feature-ok
    // File: ./examples/mut_fn/base_str.rs
    // #[cfg(feature = "ok")]
    // FIXED

    fn one<'de>(x: &'de str, y: &'de str) -> &'de str {
        println!("x = {:p}", x);
        println!("y = {:p}", y);
        if x.is_empty() {
            x
        } else {
            y
        }
    }

    let first_str = "";
    let second_str = "World";
    println!("first_str = {:p}", first_str);
    println!("second_str = {:p}", second_str);
    let result = one(first_str, second_str);

    println!();
    println!("first_str = {:p}", first_str);
    println!("second_str = {:p}", second_str);
    println!("result = {:p}", result);
    println!("result = {}", result);

    // ANCHOR_END: feature-ok
}



//=======
#[cfg(feature = "cp")]
pub fn adjoin() {
    // ANCHOR: feature-cp
    // File: ./examples/mut_fn/base_str.rs
    // #[cfg(feature = "cp")]
    // 

    fn one(x: &str, y: &str) {
        println!("x = {:p}", x);
        println!("y = {:p}", y);
    }
    
    let hello = "Hello";
    let world = "World";
    println!("hello = {:p}", hello);
    println!("world = {:p}", world);
    one(hello, world);
    
    // ANCHOR_END: feature-cp
}




//=======
#[cfg(feature = "okay")]
pub fn adjoin() {
    // ANCHOR: feature-okay
    // File: ./examples/mut_fn/base_str.rs
    // #[cfg(feature = "okay")]
    // compare to #[cfg(feature = "err_08")]

    fn one<'cn>(x: &'cn str, y: &str) -> &'cn str {
        x
    }

    let first_str = "Hello";
    let second_str = "World";
    let result = one(first_str, second_str);
    println!("result = {}", result);

    // ANCHOR_END: feature-okay
}



//=======
#[cfg(feature = "okey")]
pub fn adjoin() {
    // ANCHOR: feature-okey
    // File: ./examples/mut_fn/base_str.rs
    // #[cfg(feature = "okey")]
    // FIXED

    fn one(input: &str) {
        println!("one() input = {:p}", input);

        let ret_input = input.to_ascii_uppercase();
        println!("one() ret_input = {:p}", &ret_input);
        println!("one() ret_input = {}", ret_input);
        
        println!("one() input = {:p}", input);
        println!("one() input = {}", input);
    }
 
    let mut instance = String::new();
    instance.push_str("Hello");
    println!("after instance change = {:p}", &instance);
    println!();

    let immut_ref :&str = &instance;
    println!("before call one instance = {:p}", immut_ref);
    one(immut_ref);
    println!("after call one instance = {:p}", immut_ref);

    println!("instance = {}", instance);

    // ANCHOR_END: feature-okey
}




//=======
#[cfg(feature = "err_07")]
pub fn adjoin() {
    // ANCHOR: feature-error_07
    // File: ./examples/mut_fn/base_str.rs
    // #[cfg(feature = "err_07")]
    // error[E0106]: missing lifetime specifier
    // FIXED

    fn one(x: &str, y: &str) -> &str {
        x
    }

    let first_str = "Hello";
    let second_str = "World";
    let result = one(first_str, second_str);
    println!("result = {}", result);


    // ANCHOR_END: feature-error_07
}




//=======
#[cfg(feature = "err_08")]
pub fn adjoin() {
    // ANCHOR: feature-error_02
    // File: ./examples/mut_fn/base_str.rs
    // #[cfg(feature = "err_08")]
    // 
    // 

    fn one<'cn>(x: &'cn str, y: &str) -> &'cn str {
        y
    }

    let first_str = "Hello";
    let second_str = "World";
    let result = one(first_str, second_str);
    println!("result = {}", result);


    // ANCHOR_END: feature-error_02
}




//=======
#[cfg(feature = "err_09")]
pub fn adjoin() {
    // ANCHOR: feature-error_03
    // File: ./examples/mut_fn/base_str.rs
    // #[cfg(feature = "err_09")]
    // error[E0106]: missing lifetime specifier
    // 

    fn one(x: &str, y: &String) -> &str {
        x
    }

    let first_str = "Hello";
    let second_str = "World".to_string();
    let result = one(first_str, second_str);
    println!("result = {}", result);


    // ANCHOR_END: feature-error_03
}



//=======
#[cfg(feature = "err_10")]
pub fn adjoin() {
    // ANCHOR: feature-error_04
    // File: ./examples/mut_fn/base_str.rs
    // #[cfg(feature = "err_10")]
    // error[E0106]: missing lifetime specifier
    // 

    //fn one() -> &'static str { //OK
    fn one() -> &'de str {
        "Hello"
    }

    let result = one();
    println!("result = {}", result);

    // ANCHOR_END: feature-error_04
}



//=======
#[cfg(all(
    not(feature = "ok"),
    not(feature = "cp"),
    not(feature = "okay"),
    not(feature = "okey"),
    not(feature = "err_07"),
    not(feature = "err_08"),
    not(feature = "err_09"),
    not(feature = "err_10"),
))]
pub fn adjoin() {
    use aide::*;
    hello();
}

// https://play.rust-lang.org/?gist=cae31659a0caa396c2702039b5e16964&version=stable