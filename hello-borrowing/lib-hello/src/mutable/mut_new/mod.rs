// Goto the Project "bin-local-hello"
// cargo run --example usage -- string_mut_new
// cargo run --example usage -- vec_mut_new

pub fn use_string_mut_new() {
    // ANCHOR: use_string_mut_new
    // File: lib-hello/src/mutable/mut_new/mod.rs
    // Function use_string_mut_new()

    let immut_string = String::from("Hello");
    dbg!(immut_string);

    let mut mut_string = String::new();
    mut_string.push_str("Hello");
    dbg!(mut_string);

    // ANCHOR_END: use_string_mut_new
}

pub fn use_vec_mut_new() {
    // ANCHOR: use_vec_mut_new
    // File: lib-hello/src/mutable/mut_new/mod.rs
    // Function use_vec_mut_new()

    let immut_vec: Vec<_> = vec![33, 42];
    dbg!(immut_vec);

    let mut mut_vec: Vec<u8> = Vec::<u8>::new();
    mut_vec.push(33);
    mut_vec.push(42);
    dbg!(mut_vec);

    // ANCHOR_END: use_vec_mut_new
}