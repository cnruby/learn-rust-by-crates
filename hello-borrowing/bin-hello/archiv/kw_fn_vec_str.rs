// File: ./examples/kw_fn.rs
//

#[cfg(feature = "ok")]
fn main() {
    fn print_berry_names(berries: &Vec<&str>) {
        for berry in berries {
            println!("{}", berry);
        }
    }

    let berry_instances = vec!["Blackberry", "Strawberry"];
    print_berry_names(&berry_instances);

    dbg!(berry_instances);
}

#[cfg(feature = "err")]
// error[E0384]
fn main() {
    fn print_berry_names(berries: Vec<&str>) {
        for berry in &berries {
            println!("{}", berry);
        }
    }

    let berry_instances = vec!["Blackberry", "Strawberry"];
    print_berry_names(berry_instances);

    dbg!(berry_instances);
}

#[cfg(all(not(feature = "ok"), not(feature = "err")))]
fn main() {
    use aide::*;
    hello();
}
