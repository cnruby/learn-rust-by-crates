pub mod features;

pub fn hello() {
    println!("{}\n", "Start... Hello Borrowing!");
    println!("{}", "use source codes:");
    println!("\t\t{}", "cargo run --bin bw -- -h");
    println!("\t\tcargo run --example <RUST_FILE_NAME> --features ok");
    println!("\t\tcargo run --example <RUST_FILE_NAME> --features err");
    println!("\texample:");
    println!("\t\tcargo run --example kw_let --features ok");
    println!("\n{}", "use the crate `borrowing_exerci`:");
    println!("\t\t{}", "bw -h");
    println!("\t\t{}", "bw --help");
    println!("\t\t{}", "bw --file <RUST_FILE_NAME> --mode ok");
    println!("\t\t{}", "bw --file <RUST_FILE_NAME> --mode err");
    println!("\texample:");
    println!("\t\t{}", "bw --file kw_let --mode err | bat -l rs ");
    println!("\t\t{}", "tip: f");
    println!("\t\t{}", "tip: q");
}

pub fn allx_cmds(file_name: &str) {
    let ret = format!(
        r#"
# Show this command
cargo run --bin bw -- --file {0} | bat -l cmd
cargo install borrowing_exerci
bw --file {0}
bw --file {0} | bat -l cmd

# Show the Help for Rust File
cargo run --bin bw -- --file <FILENAME>
cargo install borrowing_exerci
bw --file <FILENAME> | bat -l cmd

# Run OK:
cargo run --bin bw -- --file <FILENAME> --mode ok
cargo run --example <FILENAME> --features ok
cargo install borrowing_exerci
bw --file <FILENAME> --mode ok

# Compile-Time Error:
cargo run --bin bw -- -f <FILENAME> -m err | bat -l rs
cargo run --example <FILENAME> --features err
cargo install borrowing_exerci
bw --file <FILENAME> -m err | bat -l rs
"#,
        file_name
    );
    println!("{}", ret);
}

pub fn devx_cmds(file_name: &str) {
    let ret = format!(
        r#"
# Show this Help for Developer
cargo run --bin bw -- --file {0} | bat -l cmd
../target/debug/bw --file {0} | bat -l cmd
cargo install borrowing_exerci
bw --file {0}
bw --file {0} | bat -l cmd

# Use the tool cargo-expand
cargo expand --example {0} --features ok
cargo expand --example {0} --features err
RUSTFLAGS="--emit mir" cargo build --release --example {0} --features ok
RUSTFLAGS="--emit mir" cargo build --release --example {0} --features err
ls -al ../target/release/examples/{0}-*.mir
open -t ../target/release/examples/{0}-*.mir

# Show the Help for User
cargo run --bin bw -- --file {0}
../target/debug/bw --file {0}
cargo install borrowing_exerci
bw --file {0} | bat -l cmd

# Run OK:
cargo run --bin bw -- --file {0} --mode ok
../target/debug/bw --file {0} --mode ok
cargo run --example {0} --features ok
cargo install borrowing_exerci
bw --file {0} --mode ok

# Compile-Time Error:
cargo run --bin bw -- -f {0} -m err | bat -l rs
../target/debug/bw --file {0} --mode err
cargo run --example {0} --features err
cargo install borrowing_exerci
bw --file {0} -m err | bat -l rs
"#,
        file_name
    );
    println!("{}", ret);
}

pub fn user_cmds(file_name: &str) {
    let ret = format!(
        r#"
// Show this command
cargo install borrowing_exerci
bw --file {0} | bat -l cmd

// Run OK:
cargo install borrowing_exerci
bw --file {0} --mode ok | bat -l rs

// Compile-Time Error:
cargo install borrowing_exerci
bw --file {0} -m err | bat -l rs
"#,
        file_name
    );
    println!("{}", ret);
}

//## 题外话
//- https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=50eaf399f641e0965e86be08a6d2d777

// https://stackoverflow.com/questions/26946646/rust-package-with-both-a-library-and-a-binary
