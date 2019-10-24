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
#[cfg(feature = "err")]
fn main() {
    let berry_instances = vec!["Blackberry", "Strawberry"];
    print_berry_names(berry_instances);

    dbg!(berry_instances);
}

#[cfg(feature = "err")]
fn print_berry_names(berries: Vec<&str>) {
    for berry in &berries {
        println!("{}", berry);
    }
}

#[cfg(all(not(feature = "ok"), not(feature = "err")))]
fn main() {
    use aide::*;
    hello();
}
