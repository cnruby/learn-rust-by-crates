// File: ./examples/closure_move.rs
// clear && cargo run --example closure_move --features ok | bat -l rs
// clear && cargo run --example closure_move --features err

#[cfg(feature = "ok")]
// ANCHOR: feature-ok
fn main() {
    let instance = vec![1,2,3];
    
    // borrow this variable `instance`
    ( || (instance.len()) )();
    //|| ( instance.len() );
    
    println!("{:?}", instance);  // ok; x was not moved
}
// ANCHOR_END: feature-ok

#[cfg(feature = "cp")]
// ANCHOR: feature-cp
fn main() {
    let instance = vec![1,2,3];
    
    // borrow this variable `instance`
    ( || { (&instance); } )(); 
    //|| { (&instance); };
    
    // ok: x was not moved
    println!("{:?}", instance);
}
// ANCHOR_END: feature-cp

#[cfg(feature = "err")]
// ANCHOR: feature-err
fn main() {
    let instance = vec![1,2,3];
    
    // move this variable `instance`
    ( move || { (&instance); } )();
    //( move || (instance.len()) )();
    //move || { (&instance); };
    //move || ( instance.len() );
    
    // ERROR: x was moved
    println!("{:?}", instance);
}
// ANCHOR_END: feature-err

#[cfg(all(not(feature = "ok"), not(feature = "err"), not(feature = "cp") ))]
fn main() {
    use aide::hello;
    hello();
}
