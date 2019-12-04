// File: ./bin-hello/examples/kw_fn/vec_str/mod.rs
// clear && cargo run --example kw_fn --features ok -- vec_str | bat -l cmd
// clear && cargo run --example kw_fn --features err_02
// clear && cargo run --example kw_fn -- vec_str

//=======



//=======
#[cfg(feature = "ok")]
pub fn adjoin() {
    // ANCHOR: feature-ok
    // File: ./bin-hello/examples/kw_fn/vec_str/mod.rs
    // #[cfg(feature = "ok")]

    fn print_berry_names(berries: &Vec<&str>) {
        for berry in berries {
            println!("{}", berry);
        }
    }

    let berry_instances = vec!["Blackberry", "Strawberry"];
    print_berry_names(&berry_instances);

    dbg!(berry_instances);

    // ANCHOR_END: feature-ok
}



//=======
#[cfg(feature = "err_02")]
// error[E0384]
pub fn adjoin() {
    // ANCHOR: feature-err
    // File: ./bin-hello/examples/kw_fn/vec_str/mod.rs
    // #[cfg(feature = "err_02")]

    fn print_berry_names(berries: Vec<&str>) {
        for berry in &berries {
            println!("{}", berry);
        }
    }

    let berry_instances = vec!["Blackberry", "Strawberry"];
    print_berry_names(berry_instances);

    dbg!(berry_instances);

    // ANCHOR_END: feature-err
}



//=======
#[cfg(all(not(feature = "ok"), not(feature = "err_02")))]
pub fn adjoin() {
    use aide::*;
    hello();
}
