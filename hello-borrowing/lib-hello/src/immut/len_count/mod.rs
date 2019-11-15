// Goto the Project "bin-local-hello"
// cargo run --example usage -- string_len_count

pub fn use_string_len_count() {
    // ANCHOR: use_string_len_count
    // File: lib-hello/src/immut/len_count/mod.rs
    // Function: use_string_len_count()
    let x = "你好";

    let x_1 = x.chars().nth(1);
    dbg!(x_1);

    let x_len = x.len();
    dbg!(x_len);

    let x_count = x.chars().count();
    dbg!(x_count);
    // ANCHOR_END: use_string_len_count
}