#[cfg(feature = "ok")]
fn main() {
    fn greet(name: &String){
        println!("Hello, {}", name);
    }

    fn goodbye(name: &String){
        println!("Goodbye, {}", name);
    }

    let peter = "Peter".to_string();
    greet(&peter);
    goodbye(&peter);
}

#[cfg(feature = "err")]
fn main() {
    fn greet(name: String){
        println!("Hello, {}", name);
    }

    fn goodbye(name: String){
        println!("Goodbye, {}", name);
    }

    let peter = "Peter".to_string();
    greet(peter);
    goodbye(peter);
}

#[cfg(all(not(feature = "ok"), not(feature = "err")))]
fn main() {
    use aide::hello;
    hello();
}