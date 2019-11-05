#![allow(unused_variables)]

fn main() {
    let instance :&str = "Hallo";
    let instance :String = instance.to_owned();
    let instance :&str = instance.as_str();
}
