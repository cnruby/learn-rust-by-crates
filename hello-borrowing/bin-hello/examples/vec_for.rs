// Run OK:
// cargo run --bin bw -- --file vec_for --mode ok
// cargo run --bin vec_for --features 'ok'
// cargo install borrowing_exerci
// bw --file vec_for --mode ok

// Compile-Time Error:
// cargo run --bin bw -- -f vec_for -m error | bat -l rs
// cargo run --bin bw -- -f vec_for
// cargo run --bin vec_for --features 'error'
// cargo install borrowing_exerci
// bw --file vec_for -m error
// bw -f vec_for

#[cfg(feature = "ok")]
fn main() {
    let a = [1, 2, 3];
    for i in a.iter() {
        print!("{} ", i);
    }
    println!("\nv = {:?}", a);

    let v = vec![1, 2, 3];
    for i in &v {
        print!("{} ", i);
    }
    println!("\nv = {:?}", v);

}

// error[E0277]
#[cfg(feature = "error")]
fn main() {
    let a = [1, 2, 3];
    for i in a {
        print!("{} ", i);
    }
    println!("\na = {:?}", a);

    let v = vec![1, 2, 3];
    for i in v {
        print!("{} ", i);
    }
    println!("v = {}", v); // error here
}

#[cfg(not(feature = "ok"))]
#[cfg(not(feature = "error"))]
fn main() {
    use aide::hello;
    hello();
}