#[cfg(feature = "ok")]
fn main() {
    let str_instance: &str = "hello";

    let greet_friend = |str: &str| println!("{} friend", str);
    greet_friend(str_instance);

    println!("{} world!", str_instance);
}

#[cfg(feature = "err")]
fn main() {
    let string_instance :String = "hello".to_string();

    let greet_friend = |string: String| println!("{} friend", string);
    greet_friend(string_instance);

    println!("{} world!", string_instance);
}

#[cfg(all(not(feature = "ok"), not(feature = "err")))]
fn main() {
    use aide::hello;
    hello();
}
