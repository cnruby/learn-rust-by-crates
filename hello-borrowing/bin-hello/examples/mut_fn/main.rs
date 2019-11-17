use std::env::args;

mod double_refs; //01

fn main() {
    match args().nth(1) {
        Some(ref x) => {
            println!("enter Mod Name: {}", x);
            match x.as_str() {
                "double_refs" => double_refs::adjoin(),
                _ => println!("No Mod thing"),
            };
        }
        None => println!("Nothing"),
    }
}

// http://xion.io/post/code/rust-examples.html#rust-examples
// https://stackoverflow.com/questions/25383488/how-to-match-a-string-against-string-literals-in-rust
