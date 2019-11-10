use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub mod features;

const STR_OK: &str = r#"#[cfg(feature = "ok")]"#;
const _STR_CP: &str = r#"#[cfg(feature = "cp")]"#;
const STR_ERR: &str = r#"#[cfg(feature = "err")]"#;
const STR_NOT: &str = r#"#[cfg(all(not(feature = "ok"), not(feature = "err")))]"#;
const _STR_NOT_ALL: &str = r#"#[cfg(all(not(feature = "ok"), not(feature = "cp"), not(feature = "err") ))]"#;
const END: &str = "\"#;";

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

pub fn convert_rs(file_name: &str) {
    // Create a path to the desired file

    let source = read_rs(file_name, false);
    let mut destination = String::new();
    let re_begin = Regex::new(r"^\n").unwrap();
    let re_end = Regex::new(r"\n\n$").unwrap();
    let re_rs_file_name = Regex::new(r"RS").unwrap();
    let begin_ok: &str = r#"pub const RS_OK :&str = r#""#;
    let begin_ok = re_rs_file_name.replace_all(begin_ok, file_name.to_uppercase().as_str());
    let begin_ok = &format!("{}", begin_ok);
    let _begin_cp: &str = r#"pub const RS_CP :&str = r#""#;
    let _begin_cp = re_rs_file_name.replace_all(begin_cp, file_name.to_uppercase().as_str());
    let _begin_cp = &format!("{}", begin_cp);
    let begin_err: &str = r#"pub const RS_ERR :&str = r#""#;
    let begin_err = re_rs_file_name.replace_all(begin_err, file_name.to_uppercase().as_str());
    let begin_err = &format!("{}", begin_err);
    let re_comment = Regex::new(r"^\s*//").unwrap();

    // https://stackoverflow.com/questions/26643688/how-do-i-split-a-string-in-rust
    let split: Vec<&str> = source.split(STR_OK).collect();
    let pre = split[0];
    if split.is_empty() {
        unimplemented!();
    } else {
        if split[0].contains("main()") {
            unimplemented!();
        } else {
            if split[1].contains("main()") {
                destination = format!("{}{}{}", destination, begin_ok, pre);
                //let split: Vec<&str> = split[1].split(STR_CP).split(STR_ERR).collect();
                let split: Vec<&str> = split[1].split(STR_ERR).collect();
                if split[0].contains("main()") {
                    //dbg!(split[0]); //ok
                    let result = re_begin.replace_all(split[0], "");
                    //let result = &format!("{}", result);  // is equal to as_ref()
                    let result = re_end.replace_all(result.as_ref(), "\n");
                    destination = format!("{}{}{}\n\n", destination, result, END);
                    let split: Vec<&str> = split[1].split(STR_NOT).collect();
                    if split[0].contains("main()") {
                        destination = format!("{}{}{}", destination, begin_err, pre);
                        //dbg!(split[0]); //err
                        let result = re_begin.replace_all(split[0], "");
                        let result = re_end.replace_all(result.as_ref(), "\n");
                        destination = format!("{}{}{}", destination, result, END);
                    } else {
                        unimplemented!();
                    }
                } else {
                    unimplemented!();
                }
            } else {
                unimplemented!();
            }
        }
    }

    // remove the comments
    //dbg!(&destination);
    let lines = destination.lines();
    let mut codes_str = String::new();
    for line in lines {
        let result = re_comment.replace_all(line, "");
        if line == result {
            codes_str = format!("{}{}\n", codes_str, result.as_ref());
        }
        //dbg!(&result);
    }

    write_rs(file_name, codes_str);
    //show_rs(file_name);
}

// https://doc.rust-lang.org/rust-by-example/std_misc/file/open.html
// https://doc.rust-lang.org/rust-by-example/std_misc/file/create.html
fn read_rs(file_name: &str, visible: bool) -> String {
    let file = format!("./examples/{}.rs", file_name);
    let path = Path::new(&file);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why.description()),
        Ok(_) => {
            if visible {
                print!("{} contains:\n{}\n\n\n", display, s);
            } else {
                print!("successfully read to\n{}\n\n", display);
            }
        }
    }
    s
}

fn write_rs(file_name: &str, stream: String) {
    // Create a path to the desired file
    let file = format!("./src/features/rs_files/{}.rs", file_name);
    let new_path = Path::new(&file);
    let new_display = new_path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut new_file = match File::create(&new_path) {
        Err(why) => panic!("couldn't create {}: {}", new_display, why.description()),
        Ok(new_file) => new_file,
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    match new_file.write_all(stream.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", new_display, why.description()),
        Ok(_) => println!("successfully wrote to\n{}\n\n", new_display),
    }
}

/*
fn show_rs(file_name: &str) {
    let file = format!("./src/features/rs_files/{}.rs", file_name);
    let new_path = Path::new(&file);
    let new_display = new_path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut new_file = match File::open(&new_path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", new_display,
                                                   why.description()),
        Ok(new_file) => new_file,
    };

    let mut s = String::new();
    match new_file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", new_display,
                                                   why.description()),
        Ok(_) => print!("{} contains:\n{}\n\n\n", new_display, s),
    }
}
*/

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
cargo expand --example {0} --features ok | bat -l cmd
cargo expand --example {0} --features err | bat -l cmd
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
cargo run --example {0} --features ok | bat -l cmd
cargo run --bin bw -- --file {0} --mode ok | bat -l cmd
../target/debug/bw --file {0} --mode ok | bat -l cmd
cargo install borrowing_exerci
bw --file {0} --mode ok

# Compile-Time Error:
cargo run --example {0} --features err | bat -l cmd
cargo run --bin bw -- -f {0} -m err | bat -l rs
../target/debug/bw --file {0} --mode err | bat -l cmd
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
bw --file <RS_FILE_NAME> --mode ok | bat -l rs

// Compile-Time Error:
cargo install borrowing_exerci
bw --file <RS_FILE_NAME> -m err | bat -l rs
"#,
        file_name
    );
    println!("{}", ret);
}

//## 题外话
//- https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=50eaf399f641e0965e86be08a6d2d777

// https://stackoverflow.com/questions/26946646/rust-package-with-both-a-library-and-a-binary

// Cow, as_ref()
// https://stackoverflow.com/questions/47147844/how-do-i-get-a-str-or-string-from-stdborrowcowstr
// https://jwilm.io/blog/from-str-to-cow/

// fn contains
// https://stackoverflow.com/questions/48794974/how-to-check-if-a-string-contains-a-substring-in-rust

// fn is_empty
// https://doc.rust-lang.org/nightly/std/vec/struct.Vec.html#method.is_empty

// regex
// https://stackoverflow.com/questions/19274493/regular-expression-match-lines-starting-with-a-certain-character-or-whitespace-a
// https://stackoverflow.com/questions/1240504/regular-expression-to-match-string-starting-with-stop

// fn split
// https://stackoverflow.com/questions/26643688/how-do-i-split-a-string-in-rust

// fn to_uppercase
// https://doc.rust-lang.org/std/char/struct.ToUppercase.html

// file read write
// https://doc.rust-lang.org/rust-by-example/std_misc/file/open.html
// https://doc.rust-lang.org/rust-by-example/std_misc/file/create.html
