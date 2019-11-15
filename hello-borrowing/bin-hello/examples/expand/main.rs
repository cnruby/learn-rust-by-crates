use std::env::args;

mod struct_str; // 07 08 09
mod struct_string; //04 05 06
mod struct_u8; //01 02 03 10
mod use_struct;
mod use_u8; //

fn main() {
    match args().nth(1) {
        Some(ref x) => {
            println!("enter Mod Name: {}", x);
            match x.as_str() {
                "struct_u8" => struct_u8::adjoin(),
                "struct_string" => struct_string::adjoin(),
                "struct_str" => struct_str::adjoin(),
                "use_u8" => use_u8::adjoin(),
                "use_struct" => use_struct::adjoin(),
                _ => println!("No Mod thing"),
            };
        }
        None => println!("Nothing"),
    }
}

// http://xion.io/post/code/rust-examples.html#rust-examples
// https://stackoverflow.com/questions/25383488/how-to-match-a-string-against-string-literals-in-rust
