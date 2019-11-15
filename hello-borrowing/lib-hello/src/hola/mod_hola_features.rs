const NAME :&str = "I am hola::mod_hola_features::";

#[cfg(feature = "ok")]
pub fn fn_hola_ok() {
    println!("{}{}", NAME, "fn_hola_ok()");
}

#[cfg(feature = "err")]
pub fn fn_hola_err() {
    println!("{}{}", NAME, "fn_hola_err()");
}

#[cfg(all(not(feature = "ok"), not(feature = "err")))]
pub fn fn_main() {
    println!("{}{}", NAME, "fn_main()");
}