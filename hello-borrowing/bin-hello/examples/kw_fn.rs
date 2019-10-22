// Run OK:
// cargo run --bin bw -- --file kw_fn --mode ok
// ../target/debug/bw --file kw_fn --mode ok | bat -l rs
// cargo install borrowing_exerci
// bw --file kw_fn --mode ok

// Compile-Time Error:
// cargo run --bin bw -- -f kw_fn -m error | bat -l rs
// cargo run --bin bw -- -f kw_fn | bat -l rs
// ../target/debug/bw -f kw_fn -m error
// ../target/debug/bw -f kw_fn
// cargo install borrowing_exerci
// bw --file kw_fn -m error
// bw -f kw_fn

#[cfg(feature = "ok")]
fn main() {
    let berry_instances = vec!["Blackberry", "Strawberry"];
    print_berry_names(&berry_instances);

    dbg!(berry_instances);
}

#[cfg(feature = "ok")]
fn print_berry_names(berries: &Vec<&str>) {
    for berry in berries {
        println!("{}", berry);
    }
}

// error[E0384]
#[cfg(feature = "error")]
fn main() {
    let berry_instances = vec!["Blackberry", "Strawberry"];
    print_berry_names(berry_instances);

    dbg!(berry_instances);
}

#[cfg(feature = "error")]
fn print_berry_names(berries : Vec<&str> ){
    for berry in &berries{
        println!("{}", berry);
    }
}

#[cfg(not(feature = "ok"))]
#[cfg(not(feature = "error"))]
fn main() {
    use aide::hello;
    hello();
}