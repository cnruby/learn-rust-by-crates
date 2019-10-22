#![allow(unused)]

// Run OK:
// cargo run --bin bw -- --file stack_head --mode ok
// cargo run --bin stack_head --features 'ok'
// cargo install borrowing_exerci
// bw --file stack_head --mode ok

// Compile-Time Error:
// cargo run --bin bw -- -f stack_head -m error | bat -l rs
// cargo run --bin bw -- -f stack_head
// cargo run --bin stack_head --features 'error'
// cargo install borrowing_exerci
// bw --file stack_head -m error
// bw -f stack_head

#[cfg(feature = "ok")]
fn main() {
    let i = 42;
    let copy_i = i;
    println!("{}", i);
    println!("{}", copy_i);

    let v = vec![1, 2, 3, 4];
    let moved_v = v.clone();
    println!("{:?}", v);
    println!("{:?}", moved_v);

    #[derive(Debug, Clone)]
    struct Pair(u8);
    let pair = Pair(1);
    println!("original: {:?}", pair);
    let moved_pair = pair.clone();
    println!("copy: {:?}", moved_pair);
    println!("original: {:?}", pair);
}

// error[E0382]
#[cfg(feature = "error")]
fn main() {
    // allocated on the stack, 
    // the actual value is copied, instead of transferring ownership.
    let i = 42;
    let copy_i = i;
    println!("{}", i);
    println!("{}", copy_i);

    // allocated on the heap, 
    // the ownership is transferred
    let v = vec![1, 2, 3, 4];
    let moved_v = v;             // value moved here
    println!("{:?}", v);    // value borrowed here after move
    println!("{:?}", moved_v);

    #[derive(Debug, Clone)]
    struct Pair(u8);
    let pair = Pair(1);
    println!("original: {:?}", pair);
    let moved_pair = pair;
    println!("copy: {:?}", moved_pair);
    println!("original: {:?}", pair);
}

#[cfg(not(feature = "ok"))]
#[cfg(not(feature = "error"))]
fn main() {
    use aide::hello;
    hello();
}