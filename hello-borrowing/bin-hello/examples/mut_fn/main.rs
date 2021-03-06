use std::env::args;

mod double_refs; //01 02 03
mod base_string; //04 05 06
mod base_str; //07 08 09

fn main() {
    match args().nth(1) {
        Some(ref x) => {
            //println!("enter Mod Name: {}", x);
            match x.as_str() {
                "base_string" => base_string::adjoin(),
                "base_str" => base_str::adjoin(),
                "double_refs" => double_refs::adjoin(),
                _ => println!("No Mod thing"),
            };
        }
        None => println!("Nothing"),
    }
}

// http://xion.io/post/code/rust-examples.html#rust-examples
// https://stackoverflow.com/questions/25383488/how-to-match-a-string-against-string-literals-in-rust
