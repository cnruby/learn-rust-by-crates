// cargo run --example hallo
// >> Nothing
// cargo run --example hallo -- hello
// >> enter Mod Name: hello
// >> Hello!
// cargo run --example hallo -- hallo
// >> enter Mod Name: hallo
// >> Hallo!
// cargo run --example hallo -- hola
// >> enter Mod Name: Hola
// >> No Mod thing

use std::env::args;

use hello_borrowing::mod_hello;
use hello_borrowing::mod_hallo;

fn main() {
    match args().nth(1) {
        Some(ref x) => {
            println!("enter Mod Name: {}", x);
            match x.as_str() {
                "hello" => mod_hello::fn_hello(),
                "hallo" => mod_hallo::fn_hallo(),
                _ => println!("No Mod thing"),
            };
        },
        None        => println!("Nothing")
    }
}


// http://xion.io/post/code/rust-examples.html#rust-examples
// https://stackoverflow.com/questions/25383488/how-to-match-a-string-against-string-literals-in-rust
