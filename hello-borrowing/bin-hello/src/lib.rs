pub mod features;

pub fn hello() {
    println!("{}\n", "Start... Hello Borrowing!");
    println!("{}", "use source codes:");
    println!("\t\t{}", "cargo run --bin bw -- -h");
    println!("\n{}", "use the crate `borrowing_exerci`:");
    println!("\t\t{}", "bw -h");
}

// https://stackoverflow.com/questions/26946646/rust-package-with-both-a-library-and-a-binary