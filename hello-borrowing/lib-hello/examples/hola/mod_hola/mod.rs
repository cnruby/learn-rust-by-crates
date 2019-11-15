#[cfg(feature = "ok")]
pub fn fn_hola() {
    println!("{}", "Hola, Feature OK!");
}

#[cfg(feature = "err")]
pub fn fn_hola() {
    println!("{}", "Hola, Feature ERROR!");
}

#[cfg(all(not(feature = "ok"), not(feature = "err")))]
pub fn fn_hola() {
    println!("{}", "Hola, No Feature!");
}