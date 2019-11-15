// Goto the Project "bin-local-hello"
// cargo run --example usage -- convert_string_str

#![allow(unused_variables)]

pub fn use_convert_string_str() {
    // ANCHOR: use_convert_string_str
    // File: lib-hello/src/other/string_str/mod.rs
    // Function use_convert_string_str()

    let instance: &str = "Hallo";
    let instance: String = instance.to_owned();
    let instance: &str = instance.as_str();

    // ANCHOR_END: use_convert_string_str
}
