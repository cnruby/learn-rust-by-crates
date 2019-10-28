fn main() {
    use aide::*;
    use std::env;

    let args: Vec<String> = env::args().collect();
    //println!("My path is {}", args[0]);
    //println!("I got {:?} arguments: {:?}", args.len() - 1, &args[1..]);

    //let path = args[0].split("/").collect::<Vec<&str>>();
    //println!("I got {:?} arguments: {:?}", path.len(), path[path.len()-1]);
    /*
    let x :Option<&String> = Some(&args[1]);
    let y = "";
    match x {
        Some(y) => all_cmds(path[path.len()-1]),
        _ => cmds(path[path.len()-1]),
    }
    */
    /*
    let x = &args[1];
    let y = String::from("");
    if x == &y {
        all_cmds(path[path.len()-1]);
    } else {
        cmds(x);
    }
    */

    //*
    let x = &args[1..];
    match { x } {
        // cargo run --bin bw -- -f bwx
        // cargo run --bin bwx
        [] => hello(),
        _ => {
            let xx: &str = &args[1];
            match xx {
                // cargo run --bin bw -- -f allx
                // cargo run --bin bwx allx
                "allx" => allx_cmds(&args[1]),
                // cargo run --bin bw -- -f user
                // cargo run --bin bwx user
                "user" => user_cmds(&args[1]),
                // cargo run --bin bw -- -f <RUST_FILE_NAME>
                // cargo run --bin bwx <RUST_FILE_NAME>
                // cargo run --bin bw -- -f kw_let
                // cargo run --bin bwx kw_let
                //"bw_ex" => run_example_bin();
                _ => devx_cmds(&args[1]),
            }
        }
    }
    //*/
}

// move the comments to alone file "features.rs"
// move the codes to this file

// 题外话
// https://stackoverflow.com/questions/26643688/how-do-i-split-a-string-in-rust

// 题外话
// https://github.com/rust-lang/rust/issues/15104
// https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html
/*
fn main() {
    match &[1,2,3] {
        [] => (),
        [_] => (),
        [_, ..] => ()  // error: unreachable pattern
    }
}

fn main() {
    match &[1,2,3] {
        [] => (),
        [_, ..] => ()  // works fine
    }
}
*/

// https://stackoverflow.com/questions/51005930/why-can-i-directly-match-an-array-of-options-but-not-a-variable-containing-an-ar
/*
fn consume(_: Box<u64>) {}
let array = [Some(Box::new(1)), Some(Box::new(2))];
match {array} {
    [Some(x), Some(y)] => {
        consume(x);
        consume(y);
    }
    _ => (),
}
*/
