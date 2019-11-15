// Goto the Project "bin-local-hello"
// cargo run --example usage -- kw_fn_u8_ref
// cargo run --example usage -- kw_fn_u8

pub fn use_kw_fn_u8_ref() {
    // ANCHOR: use_kw_fn_u8_ref
    // File: lib-hello/src/immut/kw_fn/mod.rs
    // Function use_kw_fn_u8_ref()

    fn fn_borrow(num: u8) {
        println!("inside fn = {:p}", &num);
    }

    let num = 42;
    println!("Before fn = {:p}", &num);
    fn_borrow(num);
    println!("After fn = {:p}", &num);

    dbg!(num);

    // ANCHOR_END: use_kw_fn_u8_ref
}

pub fn use_kw_fn_u8() {
    // ANCHOR: use_kw_fn_u8
    // File: lib-hello/src/immut/kw_fn/mod.rs
    // Function use_kw_fn_u8()

    fn fn_borrow(_: u8) {}

    let num = 42;
    fn_borrow(num);

    dbg!(num);
    
    // ANCHOR_END: use_kw_fn_u8
}
