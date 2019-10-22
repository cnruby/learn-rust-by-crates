#[macro_use]
extern crate clap;

fn main() {
    use clap::App;
    use std::env;
    use aide::features;

    let args: Vec<String> = env::args().collect();

    let yml = load_yaml!("borrow.yml");
    let m = App::from(yml).get_matches();

    if let Some(mode) = m.value_of("mode") {
        // cargo run --bin bw -- --mode kw_let
        // target/debug/bw --mode kw_let
        features::match_mode(&args, mode);
    } else {
        let mode = "error";
        features::match_mode(&args, mode);
        //println!("the command run: `cargo run --bin bw`");
        //println!("or, cargo install rs-borrow");
        //println!("the command run: `bw`");
    }
}
