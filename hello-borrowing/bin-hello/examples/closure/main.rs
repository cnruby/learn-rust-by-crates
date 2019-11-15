use std::env::args;

mod immut_string; //01
mod kw_move; //02
mod move_vec; //03,04

fn main() {
    match args().nth(1) {
        Some(ref x) => {
            println!("enter Mod Name: {}", x);
            match x.as_str() {
                "immut_string" => immut_string::adjoin(),
                "kw_move" => kw_move::adjoin(),
                "move_vec" => move_vec::adjoin(),
                _ => println!("No Mod thing"),
            };
        }
        None => println!("Nothing"),
    }
}

// http://xion.io/post/code/rust-examples.html#rust-examples
// https://stackoverflow.com/questions/25383488/how-to-match-a-string-against-string-literals-in-rust
