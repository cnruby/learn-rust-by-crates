#[macro_use]
extern crate clap;

fn main() {
    use aide::features;
    use aide::hello;
    use clap::App;
    use std::env;

    let args: Vec<String> = env::args().collect();

    let yml = load_yaml!("borrow.yml");
    let m = App::from(yml).get_matches();

    //dbg!(&m);
    let pos = m.value_of("pos");
    match pos {
        Some(s) => {
            let mode = m.value_of("mode");
            match mode {
                Some(m) => {
                    //dbg!(m);
                    // cargo run --bin bw -- -f kw_fn -m ok
                    // cargo run --bin bw -- -m ok -f kw_fn
                    //features::with_mode(&args, m, s);
                    features::with_script(&args, m, s);
                }
                None => {
                    //dbg!(s);
                    // cargo run --bin bw -- -f vec_for
                    features::without_mode(&args, s);
                }
            }
        }
        None => {
            //println!("No <pos> what your favorite number is.");
            // cargo run --bin bw -- --mode ok
            hello();
        }
    }
}
