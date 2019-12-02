use std::env::args;

mod for_arr; //01
mod for_vec; //02
mod for_vec_iter; //03
mod for_iter_mut; //04 05 06
mod for_into_iter; //09
mod for_enumerate; //
mod for_next; // 07 08

fn main() {
    match args().nth(1) {
        Some(ref x) => {
            println!("enter Mod Name: {}", x);
            match x.as_str() {
                "for_arr" => for_arr::adjoin(),
                "for_vec" => for_vec::adjoin(),
                "for_vec_iter" => for_vec_iter::adjoin(),
                "for_iter_mut" => for_iter_mut::adjoin(),
                "for_into_iter" => for_into_iter::adjoin(),
                "for_enumerate" => for_enumerate::adjoin(),
                "for_next" => for_next::adjoin(),
                _ => println!("No Mod thing"),
            };
        }
        None => println!("Nothing"),
    }
}

// http://xion.io/post/code/rust-examples.html#rust-examples
// https://stackoverflow.com/questions/25383488/how-to-match-a-string-against-string-literals-in-rust
