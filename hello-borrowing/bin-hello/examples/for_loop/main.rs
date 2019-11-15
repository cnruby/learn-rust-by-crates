use std::env::args;

mod for_arr; //01
mod for_vec; //02
mod for_vec_iter; //03

fn main() {
    match args().nth(1) {
        Some(ref x) => {
            println!("enter Mod Name: {}", x);
            match x.as_str() {
                "for_arr" => for_arr::adjoin(),
                "for_vec" => for_vec::adjoin(),
                "for_vec_iter" => for_vec_iter::adjoin(),
                _ => println!("No Mod thing"),
            };
        }
        None => println!("Nothing"),
    }
}

// http://xion.io/post/code/rust-examples.html#rust-examples
// https://stackoverflow.com/questions/25383488/how-to-match-a-string-against-string-literals-in-rust
