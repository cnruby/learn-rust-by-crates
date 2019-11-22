// File: ./examples/mut_immut/ref_str.rs
// clear && cargo run --example mut_immut --features ok -- ref_str | bat -l rs
// clear && cargo run --example mut_immut --features cp -- ref_str | bat -l rs
// clear && cargo run --example mut_immut --features okay -- ref_str | bat -l rs
// clear && cargo run --example mut_immut --features okey -- ref_str | bat -l rs
// clear && cargo run --example mut_immut --features err_01
// clear && cargo run --example mut_immut --features err_02
// clear && cargo run --example mut_immut --features err_03


#![allow(unused_variables)]

#[cfg(feature = "ok")]
pub fn adjoin() {
    // ANCHOR: feature-ok  AM BEST LÖSUNG
    // File: ./examples/mut_immut/ref_str.rs
    // #[cfg(feature = "ok")]

    fn one(input: &str) -> String {  
        let result = format!("{}{}", input, ", world");
        println!("one() input = {:p}", input);
        result
    }
    
    fn two(input: &str) -> String {
        let result = format!("{}{}", input, "!");
        println!("two() input = {:p}", input);
        result
    }

    let mut instance = String::new();
    println!("instance = {:p}", &instance);
    instance.push_str("Hello");
    println!();

    let mut mut_immut_ref :&str = &instance;
    println!("before call one() mut_immut_ref = {:p}", mut_immut_ref);
    let tmp_instance = one(mut_immut_ref);
    println!("after call one() mut_immut_ref = {:p}", mut_immut_ref);
    instance = tmp_instance;
    println!("instance = {:?}", instance);
    println!();

    mut_immut_ref = &instance;
    println!("mut_immut_ref = {:p}", mut_immut_ref);
    instance = two(mut_immut_ref);
    println!("instance = {:?}", instance);
    println!();

    println!("instance = {:p}", &instance);

    // ANCHOR_END: feature-ok
}




#[cfg(feature = "cp")]
pub fn adjoin() {
    // ANCHOR: feature-cp
    // File: ./examples/mut_immut/ref_str.rs
    // #[cfg(feature = "cp")]

    use std::borrow::Cow;

    fn one<'a>(input: &'a str) -> Cow<'a, str> {
        let mut buf = String::with_capacity(input.len());
        buf.push_str(input);
        buf.push_str(", world");
        buf.into()
    }

    fn two<'a>(input: &'a str) -> Cow<'a, str> {
        let mut buf = String::with_capacity(input.len());
        buf.push_str(input);
        buf.push('!');
        buf.into()
    }    

    let mut hello_from = String::new();
    hello_from.push_str("Hello");

    let mut_ref :&str = &hello_from;
    let fn_one :Cow<'_, str> = one(mut_ref);
    hello_from = fn_one.to_string();

    let mut_ref :&str = &hello_from;
    hello_from = two(mut_ref).to_string();
    
    println!("hello from {:?}", hello_from);

    // ANCHOR_END: feature-cp
}




#[cfg(feature = "okay")]
pub fn adjoin() {
    // ANCHOR: feature-okay
    // File: ./examples/mut_immut/ref_str.rs
    // #[cfg(feature = "okay")]

    fn one(input: &str) -> Box<String> {
        let result = format!("{}{}", input, ", world");
        Box::new(result)
    }

    fn two(input: &str) -> Box<String> {
        let result = format!("{}{}", input, "!");
        Box::new(result)
    }

    let mut hello_from = String::new();
    hello_from.push_str("Hello");

    let mut_ref :&str = &hello_from;
    hello_from = one(mut_ref).to_string();
    let mut_ref :&str = &hello_from;
    hello_from = two(mut_ref).to_string();
    println!("hello from {:?}", hello_from);

    let mut mut_ref :&str = &hello_from;
    hello_from = one(mut_ref).to_string();
    mut_ref = &hello_from;
    hello_from = two(mut_ref).to_string();
    println!("hello from {:?}", hello_from);

    // ANCHOR_END: feature-okay
}





#[cfg(feature = "okey")]
pub fn adjoin() {
    // ANCHOR: feature-okey
    // File: ./examples/mut_immut/ref_str.rs
    // #[cfg(feature = "okey")]


    // ANCHOR_END: feature-okey
}





#[cfg(feature = "err_01")]
pub fn adjoin() {
    // ANCHOR: feature-error_01
    // File: ./examples/mut_immut/ref_str.rs
    // #[cfg(feature = "err_01")]

    fn one(input: &str) -> Box<String> {
        let result = format!("{}{}", input, ", world");
        Box::new(result)
    }

    fn two(input: &str) -> Box<String> {
        let result = format!("{}{}", input, "!");
        Box::new(result)
    }

    let mut hello_from = String::new();
    hello_from.push_str("Hello");

    let mut_ref :&str = &hello_from;
    hello_from = one(mut_ref).to_string();
    // mut_ref = &hello_from;  // ERROR
    hello_from = two(mut_ref).to_string();
    println!("hello from {:?}", hello_from);

    // ANCHOR_END: feature-error_01
}

#[cfg(feature = "err_02")]
pub fn adjoin() {
    // ANCHOR: feature-error_02  OK!!!!
    // File: ./examples/mut_immut/ref_str.rs
    // #[cfg(feature = "err_02")]
    


    // ANCHOR_END: feature-error_02
}




#[cfg(feature = "err_03")]
pub fn adjoin() {
    // ANCHOR: feature-error_03
    // File: ./examples/mut_immut/ref_str.rs
    // #[cfg(feature = "err_03")]
    
    fn manipulate(input: &mut String) {                                                                                                                                                                                                          
        input.push('!');
      }
 
    let mut instance = String::new();
    instance.push_str("Hello");
    let mut_ref = &mut instance;
    manipulate(mut_ref);          
    println!("{}", instance);    

    // ANCHOR_END: feature-error_03
}



#[cfg(all(
    not(feature = "ok"),
    not(feature = "cp"),
    not(feature = "okay"),
    not(feature = "okey"),
    not(feature = "err_01"),
    not(feature = "err_02"),
    not(feature = "err_03"),
    not(feature = "err_04"),
    not(feature = "err_05"),
    not(feature = "err_06"),
))]
pub fn adjoin() {
    use aide::*;
    hello();
}

//
// https://stackoverflow.com/questions/42248444/return-str-instead-of-stdborrowcow-str
// https://hermanradtke.com/2015/05/29/creating-a-rust-function-that-returns-string-or-str.html
// https://hermanradtke.com/2015/05/03/string-vs-str-in-rust-functions.html

// https://stackoverflow.com/questions/30154541/how-do-i-concatenate-strings
// https://users.rust-lang.org/t/best-way-to-do-string-concatenation-in-2019-status-quo/24004/4
// https://users.rust-lang.org/t/fast-string-concatenation/4425/2
// http://dnsh.io/music/2016/10/06/string-concatenation-in-rust-is-not-tivial/

