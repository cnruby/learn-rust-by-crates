// File: ./examples/dbg/mut_macro.rs
// clear && cargo run --example dbg --features ok -- mut_macro | bat -l rs
// clear && cargo run --example dbg --features cp -- mut_macro | bat -l rs
// clear && cargo run --example dbg --features err_01
// clear && cargo run --example dbg --features err_02
// clear && cargo run --example dbg --features err_03
// clear && cargo run --example dbg --features err_04


#[cfg(feature = "ok")]
pub fn adjoin() {
    // ANCHOR: feature-ok
    // File: ./examples/dbg/mut_macro.rs
    // #[cfg(feature = "ok")]

    let mut string :String = format!("{}", "Hello");
    string.push('!');
    let ref_string = &string;
    dbg!(ref_string);
    dbg!(ref_string);

    // ANCHOR_END: feature-ok
}



#[cfg(feature = "cp")]
pub fn adjoin() {
    // ANCHOR: feature-cp
    // File: ./examples/dbg/mut_macro.rs
    // #[cfg(feature = "cp")]

    let string :String = format!("{}", "Hello");
    dbg!(std::mem::size_of_val(&string));
    dbg!(&string);
    dbg!(string);

    // ANCHOR_END: feature-cp
}



#[cfg(feature = "err_01")]
pub fn adjoin() {
    // ANCHOR: feature-error_01
    // File: ./examples/dbg/mut_macro.rs
    // ANCHOR = "error_01"
    // error[E0382]: borrow of moved value: `string`

    let string = format!("{}", "Hello");
    dbg!(string);
    dbg!(&string);

    // ANCHOR_END: feature-error_01
}



#[cfg(feature = "err_02")]
pub fn adjoin() {
    // ANCHOR: feature-error_02
    // File: ./examples/dbg/mut_macro.rs
    // ANCHOR = "error_02"
    // error[E0382]: use of moved value: `string`

    let string = format!("{}", "Hello");
    dbg!(string);
    dbg!(string);

    // ANCHOR_END: feature-error_02
}



#[cfg(feature = "err_03")]
pub fn adjoin() {
    // ANCHOR: feature-error_03
    // File: ./examples/dbg/mut_macro.rs
    // ANCHOR = "error_03"
    // error[E0382]: use of moved value: `ref_mut_string`

    let mut string = format!("{}", "Hello");
    string.push('!');
    let ref_mut_string = &mut string;
    dbg!(ref_mut_string);
    dbg!(ref_mut_string);

    // ANCHOR_END: feature-error_03
}



#[cfg(feature = "err_04")]
pub fn adjoin() {
    // ANCHOR: feature-error_04
    // File: ./examples/dbg/mut_macro.rs
    // ANCHOR = "error_04"
    // borrow of moved value: `ref_mut_string`

    let mut string = format!("{}", "Hello");
    string.push('!');
    let ref_mut_string = &mut string;
    dbg!(ref_mut_string);
    println!("ref_mut_string = {}", ref_mut_string); // ref_mut_string is moved

    // ANCHOR_END: feature-error_04
}



#[cfg(all(
    not(feature = "ok"),
    not(feature = "cp"),
    not(feature = "err_01"),
    not(feature = "err_02"),
    not(feature = "err_03"),
    not(feature = "err_04"),
))]
fn main() {
    use aide::*;
    hello();
}
