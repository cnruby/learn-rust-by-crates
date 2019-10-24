#[cfg(feature = "ok")]
fn main() {
    let str = format!("{}", "Hello");
    dbg!(str);
}

#[cfg(feature = "err")]
fn main() {
    let str = format!("{}", "Hello");
    dbg!(str);
    dbg!(&str);
}

#[cfg(all(not(feature = "ok"), not(feature = "err")))]
fn main() {
    use aide::*;
    hello();
}
