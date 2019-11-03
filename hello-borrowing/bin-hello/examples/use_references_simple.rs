// File ./exmaples/use_references_simple.rs
// clear && cargo run --example use_references_simple | bat -l cmd
#![allow(unused_variables)]

fn main() {
    let instance :&str = "Hello";
    let instance = "Hello";

    let copy_instance :&str = instance;
    let copy_instance = instance;

    println!("instance reference address = {:p}", instance);
    println!("copy_instance reference address = {:p}", copy_instance);

    println!("instance address = {:p}", &instance);
    println!("copy_instance address = {:p}", &copy_instance);
}
