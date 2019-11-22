use std::env::args;

mod ref_str;    //01 02 03
mod mut_string; //04 05 06

fn main() {
    match args().nth(1) {
        Some(ref x) => {
            //println!("enter Mod Name: {}", x);
            match x.as_str() {
                "ref_str" => ref_str::adjoin(),
                "mut_string" => mut_string::adjoin(),
                _ => println!("No Mod thing"),
            };
        }
        None => println!("Nothing"),
    }
}

// http://xion.io/post/code/rust-examples.html#rust-examples
// https://stackoverflow.com/questions/25383488/how-to-match-a-string-against-string-literals-in-rust
