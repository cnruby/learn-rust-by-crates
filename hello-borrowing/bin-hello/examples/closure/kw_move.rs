// File: ./bin-hello/examples/closure/kw_move/mod.rs
// clear && cargo run --example closure --features ok -- kw_move | bat -l cmd
// clear && cargo run --example closure --features err_02
// clear && cargo run --example closure -- kw_move

//=======


//=======
#[cfg(feature = "ok")]
pub fn adjoin() {
    // ANCHOR: feature-ok
    // File: ./bin-hello/examples/closure/kw_move/mod.rs
    // #[cfg(feature = "ok")]

    let instance = vec![1, 2, 3];

    // borrow this variable `instance`
    (|| (instance.len()))();
    //|| ( instance.len() );

    println!("{:?}", instance); // ok; x was not moved

    // ANCHOR_END: feature-ok
}


//=======
#[cfg(feature = "cp")]
pub fn adjoin() {
    // ANCHOR: feature-cp
    // File: ./bin-hello/examples/closure/kw_move/mod.rs
    // #[cfg(feature = "cp")]

    let instance = vec![1, 2, 3];

    // borrow this variable `instance`
    (|| {
        (&instance);
    })();
    //|| { (&instance); };

    // ok: x was not moved
    println!("{:?}", instance);

    // ANCHOR_END: feature-cp
}


//=======
#[cfg(feature = "err_02")]
pub fn adjoin() {
    // ANCHOR: feature-err
    // File: ./bin-hello/examples/closure/kw_move/mod.rs
    // #[cfg(feature = "err_02")]

    let instance = vec![1, 2, 3];

    // move this variable `instance`
    (move || {
        (&instance);
    })();
    //( move || (instance.len()) )();
    //move || { (&instance); };
    //move || ( instance.len() );

    // ERROR: x was moved
    println!("{:?}", instance);

    // ANCHOR_END: feature-err
}


//=======
#[cfg(all(not(feature = "ok"), not(feature = "err_02"), not(feature = "cp")))]
pub fn adjoin() {
    use aide::hello;
    hello();
}
