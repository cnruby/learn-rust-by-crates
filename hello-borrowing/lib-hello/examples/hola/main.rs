// cargo run --example hallo
// >> Nothing
// cargo run --example hallo -- hello
// >> enter Mod Name: hello
// >> Hello!
// cargo run --example hallo -- hallo
// >> enter Mod Name: hallo
// >> Hallo!
// cargo run --example hallo -- nihao
// >> enter Mod Name: nihao
// >> No Mod thing

// cargo run --example hola --features ok -- mod_hola
// >> enter Mod Name: hola
// >> Hola, Feature OK!
// cargo run --example hola --features err -- mod_hola
// >> enter Mod Name: hola
// >> Hola, Feature ERROR!
// cargo run --example hola -- mod_hola
// >> enter Mod Name: hola
// >> Hola, No Feature!

use std::env::args;

mod mod_hello;
mod mod_hallo;
mod mod_hola;

fn main() {
    match args().nth(1) {
        Some(ref x) => {
            println!("enter Mod Name: {}", x);
            match x.as_str() {
                "hello" => mod_hello::fn_hello(),
                "hallo" => mod_hallo::fn_hallo(),
                "mod_hola" => mod_hola::fn_hola(),
                _ => println!("No Mod thing"),
            };
        },
        None        => println!("Nothing")
    }
}


// http://xion.io/post/code/rust-examples.html#rust-examples
// https://stackoverflow.com/questions/25383488/how-to-match-a-string-against-string-literals-in-rust
