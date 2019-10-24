#[cfg(feature = "ok")]
fn main() {
    #[derive(Clone, Copy)]
    struct Struct(u8);

    let instance = Struct(42u8);
    let cln_instance = instance.clone(); // Clone
    let cp_instance = instance; // Copy
    dbg!(instance.0, cp_instance.0, cln_instance.0);
}

#[cfg(feature = "err")]
fn main() {
    #[derive(Clone)]
    struct Struct(u8);

    let instance = Struct(42u8);
    let cln_instance = instance.clone(); // Clone
    let cp_instance = instance; // Copy
    dbg!(instance.0, cp_instance.0, cln_instance.0);
}

#[cfg(all(not(feature = "ok"), not(feature = "err")))]
fn main() {
    use aide::*;
    hello();
}
