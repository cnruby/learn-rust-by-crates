// File: ./examples/mut_immut/mut_string.rs
// clear && cargo run --example mut_immut --features ok -- mut_string | bat -l rs
// clear && cargo run --example mut_immut --features okay -- mut_string | bat -l rs
// clear && cargo run --example mut_immut --features okey -- mut_string | bat -l rs
// clear && cargo run --example mut_immut --features err_01
// clear && cargo run --example mut_immut --features err_02
// clear && cargo run --example mut_immut --features err_03
// clear && cargo run --example mut_immut --features err_04


#![allow(unused_variables)]

#[cfg(feature = "ok")]
pub fn adjoin() {
    // ANCHOR: feature-ok  AM BEST LÖSUNG
    // File: ./examples/mut_immut/mut_string.rs
    // #[cfg(feature = "ok")]

    fn one(input: &str) -> String {  
        let result = format!("{}{}", input, "?");
        println!("two() input = {:p}", &input);
        result
    }
    
    fn two(input: &str) -> String {
        let result = format!("{}{}", input, "?");
        println!("two() input = {:p}", &input);
        result
    }

    let mut instance = String::new();
    println!("instance = {:p}", &instance);

    instance.push_str("Hello");
    println!("instance = {:p}", &instance);

    instance = one(&instance);
    println!("instance = {:p}", &instance);

    instance = two(&instance);
    println!("instance = {:p}", &instance);

    println!("instance = {:?}", instance);

    // ANCHOR_END: feature-ok
}

#[cfg(feature = "cp")]
pub fn adjoin() {
    fn one(mut hello_from: String) -> String {
        hello_from.push('!');
        hello_from
    }
    
    fn two(mut hello_from: String) -> String {
        hello_from.push('?');
        hello_from
    }

    let mut hello_from = String::new();
    hello_from.push_str("Hello");
    hello_from = one(hello_from);
    hello_from = two(hello_from);
    println!("hello from {:?}", hello_from);
}




#[cfg(feature = "okay")]
pub fn adjoin() {
    // ANCHOR: feature-okay
    // File: ./examples/mut_immut/mut_string.rs
    // #[cfg(feature = "okay")]

    fn one(input: &str) -> Box<String> {
        let result = format!("{}{}", input, "?");
        Box::new(result)
    }

    fn two(input: &str) -> Box<String> {
        let result = format!("{}{}", input, "!");
        Box::new(result)
    }

    let mut hello_from = String::new();
    hello_from.push_str("Hello");

    let mut mut_ref :&str = &hello_from;
    hello_from = one(mut_ref).to_string();
    mut_ref = &hello_from;
    hello_from = two(mut_ref).to_string();
    println!("hello from {:?}", hello_from);

    let mut_ref :&str = &hello_from;
    hello_from = one(mut_ref).to_string();
    let mut_ref :&str = &hello_from;
    hello_from = two(mut_ref).to_string();
    println!("hello from {:?}", hello_from);

    // ANCHOR_END: feature-okay
}





#[cfg(feature = "okey")]
pub fn adjoin() {
    // ANCHOR: feature-okey
    // File: ./examples/mut_immut/mut_string.rs
    // #[cfg(feature = "okey")]

    fn one(input: &mut String) -> Box<String> {
        input.push('?');
        Box::new(input.to_string())
    }

    fn two(input: &mut String) -> Box<String> {
        input.push('!');
        Box::new(input.to_string())
    }    

    let mut mut_instance = String::new();
    mut_instance.push_str("Hello");
    
    let mut mut_ref :&mut String = &mut mut_instance;
    mut_instance = one(mut_ref).to_string();
    
    mut_ref = &mut mut_instance;
    mut_instance = two(mut_ref).to_string();

    println!("hello from {:?}", mut_instance);

    // ANCHOR_END: feature-okey
}





#[cfg(feature = "err_01")]
pub fn adjoin() {
    // ANCHOR: feature-error_01
    // File: ./examples/mut_immut/mut_string.rs
    // #[cfg(feature = "err_01")]

    fn one(hello_from: &mut String) -> &String {
        hello_from.push('!');
        hello_from
    }
    
    fn two(hello_from: &mut String) -> &String {
        hello_from.push('?');
        hello_from
    }

    let mut hello_from = String::new();
    hello_from.push_str("Hello");
    //let mut_ref :&mut String = &mut hello_from; //OK
    //let mut_ref = &mut hello_from; //OK
    let mut_ref :&String = &mut hello_from; //ERROR
    one(mut_ref);
    two(mut_ref);
    println!("hello from {:?}", hello_from);

    // ANCHOR_END: feature-error_01
}

#[cfg(feature = "err_02")]
pub fn adjoin() {
    // ANCHOR: feature-error_02  OK!!!!
    // File: ./examples/mut_immut/mut_string.rs
    // #[cfg(feature = "err_02")]
    
    fn one(input: &str) -> Box<String> {
        let mut buf = String::with_capacity(input.len());
        buf.push_str(input);
        buf.push('!');
        buf.into()
    }

    fn two(input: &str) -> Box<String> {
        let mut buf = String::with_capacity(input.len());
        buf.push_str(input);
        buf.push('!');
        buf.into()
    }


    let mut hello_from = String::new();
    hello_from.push_str("Hello");
    let mut_ref :&str = &hello_from;
    hello_from = one(mut_ref).to_string();
    // let mut_ref :&str = &hello_from;  // OK
    hello_from = two(mut_ref).to_string();
    println!("hello from {:?}", hello_from);

    // ANCHOR_END: feature-error_02
}




#[cfg(feature = "err_03")]
pub fn adjoin() {
    // ANCHOR: feature-error_03
    // File: ./examples/mut_immut/mut_string.rs
    // #[cfg(feature = "err_03")]
    
    use std::borrow::Cow;

    fn one<'a>(input: &'a str) -> Cow<'a, str> {
        let mut buf = String::with_capacity(input.len());
        buf.push_str(input);
        buf.push('!');
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

    // let mut_ref :&str = &hello_from; //OK
    hello_from = two(mut_ref).to_string();
    
    println!("hello from {:?}", hello_from);

    // ANCHOR_END: feature-error_03
}


#[cfg(feature = "err_04")]
pub fn adjoin() {
    // ANCHOR: feature-error_04
    // File: ./examples/mut_immut/mut_string.rs
    // #[cfg(feature = "err_04")]
    
    fn one(input: &str) -> Box<String> {
        let result = [input, "!"].join("");
        println!("one() input = {:p}", &input);
        let result = [input, "!"].concat();
        println!("one() input = {:p}", &input);
        Box::new(result)
    }

    fn two(input: &str) -> Box<String> {
        let result = [input, "!"].join("");
        println!("one() input = {:p}", &input);
        let result = [input, "!"].concat();
        println!("one() input = {:p}", &input);
        Box::new(result)
    }    

    let mut hello_from = String::new();
    hello_from.push_str("Hello");
    //let mut_ref :&mut str = &hello_from; ERROR
    
    //let mut_ref :&str = &mut hello_from; // OK
    let mut_ref :&str = &hello_from; // OK

    //let mut_ref :&mut str = &mut hello_from; // OK
    
    //let mut mut_ref :&str = &hello_from; // OK
    //let mut mut_ref :&str = &mut hello_from;  // OK, warning
    //let mut mut_ref :&mut str = &mut hello_from;  // OK, warning

    hello_from = one(mut_ref).to_string();
    
    // let mut_ref :&str = &hello_from; // OK
    hello_from = two(mut_ref).to_string();

    println!("hello from {:?}", hello_from);

    // ANCHOR_END: feature-error_04
}



#[cfg(feature = "err_05")]
pub fn adjoin() {
    // ANCHOR: feature-error_05
    // File: ./examples/mut_immut/mut_string.rs
    // #[cfg(feature = "err_05")]

    fn one(hello_from: &mut String) -> &String {
        hello_from.push('!');
        hello_from
    }
    
    fn two(hello_from: &mut String) -> &String {
        hello_from.push('?');
        hello_from
    }

    let mut hello_from = String::new();
    hello_from.push_str("Hello");
    //let mut_ref :&mut String = &mut hello_from; // OK
    let mut_ref :&String = &mut hello_from;  // ERROR
    one(mut_ref);
    two(mut_ref);
    println!("hello from {:?}", hello_from);

    // ANCHOR_END: feature-error_05
}

#[cfg(feature = "err_06")]
pub fn adjoin() {
    // ANCHOR: feature-error_06
    // File: ./examples/mut_immut/mut_string.rs
    // #[cfg(feature = "err_06")]

    // fn one(mut hello_from: String) -> String { // OK
    fn one(hello_from: String) -> String {
        hello_from.push('!');
        hello_from
    }
    
    // fn two(mut hello_from: String) -> String { // OK
    fn two(hello_from: String) -> String {
        hello_from.push('?');
        hello_from
    }

    let mut hello_from = String::new();
    hello_from.push_str("Hello");
    hello_from = one(hello_from);
    hello_from = two(hello_from);
    println!("hello from {:?}", hello_from);

    // ANCHOR_END: feature-error_06
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

