// Goto the Project "bin-local-hello"
// cargo run --example usage -- clone_array

pub fn use_clone_array() {
    // ANCHOR: use_clone_array
    // File: lib-hello/src/other/clone.rs
    // Function use_clone_array()

    let a: [u8; 0] = [];
    dbg!(a);
    let b = a.clone();
    let c = a;
    dbg!(a, b, c);

    let a: [u8; 0] = [];
    dbg!(a);
    let b = a;
    let c = a.clone();
    dbg!(a, b, c);
    
    // ANCHOR_END: use_clone_array
}
