#![allow(unused_must_use)]

use regex::Regex;
use std::error::Error;
use std::io::prelude::*;
use std::path::Path;
use std::borrow::Cow;
use std::fs::File;
use std::fs;

pub mod features;

const SPLIT_STR: &str = r#"//======="#;
const FEATURE_STR: &str = r#"#[cfg(feature = "#;

const END: &str = "\"#;";
const FN_MAIN: &str = "adjoin";


pub fn convert_all_rss() {
    let path_str :&str = &format!("./examples");
    let paths = fs::read_dir(path_str).unwrap();
    for path in paths {
        let p_name = path.unwrap().path();
        //println!("path = {:?}", p_name.to_str());
        match p_name.to_str() {
            Some(folder_name) => {
                if !folder_name.contains(".rs") {
                    //println!("folder_name = {:?}", folder_name);
                    let names: Vec<&str> = folder_name.split("/").collect();
                    println!("folder_name = {:?}", names[2]);
                    convert_rss(names[2]);
                }
            },
            None => {}
        }
    }
}


pub fn convert_rss(folder_name: &str) {
    let path_str :&str = &format!("./examples/{}", folder_name);
    let paths = fs::read_dir(path_str).unwrap();

    for path in paths {
        //println!("path = {}", path.unwrap().path().display());
        //let file_name :&str = path.unwrap().path().into_os_string();
        let p = path.unwrap().path();
        let f_name = p.file_name();
        match f_name {
            Some(file_name) => {
                if file_name != "main.rs" {
                    let str = file_name.to_string_lossy();
                    let str :&str = &format!("{}", str);
                    let names: Vec<&str> = str.split(".").collect();
                    println!("file_name = {:?}", names[0]);
                    convert_rs(folder_name, names[0]);
                }                
            },
            None => {}
        }
    }
}



pub fn convert_rs(folder_name: &str, file_name: &str) {
    let path_file :&str = &format!("./examples/{}/{}.rs", folder_name, file_name);
    let source = read_rs(path_file, false);

    //let destination = get_main(source, feature_name);
    let mut split: Vec<&str> = source.split(SPLIT_STR).collect();
    
    let mut codes_str = String::new();
    let split_len = split.len() - 1;
    if split_len < 4 {
        return;
    }
    let split1 = split[1];
    for (index, item) in split.iter_mut().enumerate() {        
        if index > 1 && index < split_len {
            let mut f = String::new();
            let feature_name = &mut f;
            let destination = format!("{}\n{}", split1, item);
            let mut tmp = create_rs(destination, feature_name);
            
            //println!("feature_name = {:?}", feature_name);
            let begin_str = get_begin(file_name, feature_name);
            tmp = format!("{}{}{}", begin_str, tmp, END);
            codes_str = format!("{}\n\n{}", codes_str, tmp);

            let path_file :&str = &format!("./src/features/rs_files.rs");
            insert_mod_name(path_file, folder_name, file_name);
            insert_code_name(path_file, folder_name, file_name, feature_name);
        }
    }

    let path_file :&str = &format!("./src/features/rs_files/{}_{}.rs", folder_name, file_name);
    write_rs(path_file, codes_str);
    //show_rs(file_name);

    let path_file :&str = &format!("./src/bin/borrow.yml");
    insert_example_name(path_file, folder_name, file_name);
}



fn get_begin<'de>(file_name: &'de str, feature_name: &'de str) -> Cow<'de, str>{
    const BEGIN: &str = r#"pub const ------- :&str = r#""#;
    let re_feature = Regex::new(r"-------").unwrap();
    let replace_str = format!(
        "{}_{}",
        file_name.to_uppercase(),
        feature_name.to_uppercase()
    );
    let begin_str = re_feature.replace(
        BEGIN, replace_str.as_str()
    );
    begin_str
}


fn create_rs(destination :String, feature_name :&mut String) -> String {
    let re_main = Regex::new(r"adjoin").unwrap();
    let re_comment = Regex::new(r"^\s*//").unwrap();

    let lines = destination.lines();
    let mut codes_str = String::new();
    let mut result;
    for line in lines {
        if line.contains(FN_MAIN) {
            result = re_main.replace(line, "main");
            codes_str = format!("{}{}\n", codes_str, result.as_ref());
        } else if line.contains(FEATURE_STR) {
            if line[0..10] == FEATURE_STR[0..10] {
                let tmp: Vec<&str> = line.split("\"").collect();
                //println!("tmp = {:?}", tmp);
                feature_name.push_str(tmp[1]);
            }
        } else {
            if !line.is_empty() {
                result = re_comment.replace(line, "");
                if line == result {                
                    codes_str = format!("{}{}\n", codes_str, result.as_ref());
                }
            }
        }
        //dbg!(&result);
    }

    codes_str
}

fn insert_example_name(path_file :&str, folder_name :&str, file_name :&str) {
    let mut source = read_rs(path_file, false);
    let code_name = format!( "- {}_{}", folder_name, file_name );
    
    if !source.contains(&code_name) {
    //if !source.contains(&code_name) {
        const RE_STR: &str = r#"\#\#example_name"#;
        const MUSTER_STR: &str = r#"##example_name"#;
        // - closure_move_vec
        let path_code_name :&str = &format!(
            r#"{}
            {}"#,
            MUSTER_STR,
            code_name            
        );
        println!("path_code_name = {}", path_code_name);
        let re_code = Regex::new(RE_STR).unwrap();
        let result = re_code.replace_all(&source, path_code_name);
        source = format!("{}", result);

        write_rs(path_file, source);
    } else {
        println!("NOTHING insert_example_name !!!");
    }
}

fn insert_code_name(path_file :&str, folder_name :&str, file_name :&str, feature_name :&str) {
    let mut source = read_rs(path_file, false);
    let code_name = format!(
        "{}_{}_{}",
        folder_name,
        file_name,
        feature_name
    );

    if !source.contains(&code_name) {
        // "vec_for_err" => vec_for::VEC_FOR_ERR,
        let comment_str: &str = &format!(r#"//{}"#, feature_name);
        let re_str: &str = &format!("//{}\n", feature_name);
        let path_code_name :&str = &format!(
            r#""{}" => {}_{}::{}_{},
        {}
        "#,
            code_name,
            folder_name,
            file_name,
            file_name.to_uppercase(),
            feature_name.to_uppercase(),
            comment_str,
        );
        println!("path_code_name = {}", path_code_name);
        let re_code = Regex::new(re_str).unwrap();
        let result = re_code.replace_all(&source, path_code_name);
        source = format!("{}", result);

        write_rs(path_file, source);
    } else {
        //println!("NOTHING insert_code_name !!!");
    }
}



fn insert_mod_name(path_file :&str, folder_name :&str, file_name :&str) {
    let mut source = read_rs(path_file, false);
    let code_name = format!(
        "mod {}_{};",
        folder_name,
        file_name
    );

    if !source.contains(&code_name) {
        // mod closure_move_vec
        let comment_str: &str = &format!(r#"//mod"#);
        let re_str: &str = &format!("//mod\n");
        let path_code_name :&str = &format!(
            r#"{}
{}
        "#,
            code_name,
            comment_str,
        );
        println!("path_code_name = {}", path_code_name);
        let re_code = Regex::new(re_str).unwrap();
        let result = re_code.replace_all(&source, path_code_name);
        source = format!("{}", result);

        write_rs(path_file, source);
    } else {
        //println!("NOTHING insert_mod_name !!!");
    }
}



// https://doc.rust-lang.org/rust-by-example/std_misc/file/open.html
// https://doc.rust-lang.org/rust-by-example/std_misc/file/create.html
fn read_rs(file: &str, visible: bool) -> String {
    let path = Path::new(file);
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
                print!("\n\n\n{} contains:\n{}\n\n\n", display, s);
            } else {
                print!("successfully read to\n{}\n\n", display);
            }
        }
    }
    s
}

fn write_rs(file: &str, stream: String) {
    // Create a path to the desired file    
    let new_path = Path::new(file);
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
cargo run --bin bw -- --code {0} | bat -l cmd
cargo install borrowing_exerci
bw --code {0}
bw --code {0} | bat -l cmd

# Show the Help for Rust File
cargo run --bin bw -- --code <CODE>
cargo install borrowing_exerci
bw --code <CODE> | bat -l cmd

# Run OK:
cargo run --bin bw -- --code <CODE> --feature ok
cargo run --example <CODE> --features ok
cargo install borrowing_exerci
bw --code <CODE> --feature ok

# Compile-Time Error:
cargo run --bin bw -- -c <CODE> -f err | bat -l rs
cargo run --example <CODE> --features err
cargo install borrowing_exerci
bw --code <CODE> -f err | bat -l rs
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
bw --code {0} | bat -l cmd

// Run OK:
cargo install borrowing_exerci
bw --code <CODE> --feature ok | bat -l rs

// Compile-Time Error:
cargo install borrowing_exerci
bw --code <CODE> -f err | bat -l rs
"#,
        file_name
    );
    println!("{}", ret);
}

pub fn hello() {
    println!("{}", "Start... Hello Borrowing!");
    println!("\n{}", "use this help for crate `borrowing_exerci`:");
    println!("\t{}", "bw -h");
    println!("\t{}", "bw --help");
    println!("\n\n{}", "list all faetures for a code:");
    println!("\t{}", "bw -- -c <code>");
    println!("example:");
    println!("\t{}", "bw --code closure_move_vec");
    println!("\n\n{}", "run code for the `code` and the `feature`:");
    println!("\t{}", "bw --code <code> --feature <feature>");
    println!("example:");
    println!("\t{}", "bw --code closure_move_vec --feature ok | bat -l rs ");
    println!("\t{}", "tip: f");
    println!("\t{}", "tip: q");
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
