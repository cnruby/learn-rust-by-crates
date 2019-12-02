use run_script::ScriptOptions;
mod rs_files;

pub fn with_script(args: &Vec<String>, mode: &str, file_name: &str) {
    let options = ScriptOptions::new();
    if !mode.is_empty() {
        let code_name :&str = &format!("{}_{}", file_name, mode);
        let rs_code = rs_files::get_rs(code_name);
        let set_file = format!("echo '{}'", rs_code);
        let cmd_echo = format!("echo '{}' > {}.rs", rs_code, file_name);
        let cargo_script = format!("cargo script {}.rs", file_name);
        let rm_script = format!("rm -rf {}.rs", file_name);
        //dbg!(&rs_code);
        //dbg!(&cargo_script);
        //let cmds = format!("{}", &set_file);
        let cmds = format!(
            "{}\n{}\n{}\n{}",
            &set_file, &cmd_echo, &cargo_script, &rm_script
        );
        let (_code, output, error) = run_script::run(&cmds, &args, &options).unwrap();
        //println!("Exit Code: {}\n\n", code);
        println!("{}\n\n", output);
        println!("Compiler: Output Info  >>>>>>>>>>>>>> :\n\n{}", error);
    }
}

/*
pub fn with_mode(args: &Vec<String>, mode: &str, file_name: &str) {
    let options = ScriptOptions::new();
    let rs_file: &str = &format!("RS_FILE=./examples/{}.rs", file_name);
    let cmd_cargo;
    // cargo run --bin bw -- -f kw_let -m error | bat -l rs
    // cargo run --bin bw -- --file kw_let --mode ok
    // cargo run --bin bw -- --file kw_let --mode err
    // is equal to
    // # cargo run --example kw_let --features ok
    // # cargo run --example kw_let --features err
    cmd_cargo = format!("cargo run --example {} --features '{}'", file_name, mode);
    dbg!(&cmd_cargo);
    let cmds = format!("{}\n{}\n{}", rs_file, &cmd_cargo, RUM_CMD_BAT);
    let (_code, output, error) = run_script::run(&cmds, &args, &options).unwrap();
    //println!("Exit Code: {}\n\n", code);
    println!("{}\n\n", output);
    println!("Compiler: Output Info  >>>>>>>>>>>>>> :\n\n{}", error);
}
*/

pub fn without_mode(args: &Vec<String>, file_name: &str) {
    let options = ScriptOptions::new();
    let cmd_cargo;
    match file_name.as_ref() {
        "bwx" => {
            // cargo run --bin bw -- -f bwx
            // # cargo run --bin bwx
            cmd_cargo = format!("cargo run --bin {}", file_name);
            dbg!(&cmd_cargo);
            let cmds = format!("{}", &cmd_cargo);
            let (_code, output, error) = run_script::run(&cmds, &args, &options).unwrap();
            println!("{}", output);
            dbg!(error);
        }
        _ => {
            // cargo run --bin bw -- -f kw_let
            // cargo run --bin bw -- -f allx
            // cargo run --bin bw -- -f devx
            // # cargo run --bin bwx kw_let
            // # cargo run --bin bwx allx
            // # cargo run --bin bwx devx
            cmd_cargo = format!("cargo run --bin bwx {}", file_name);
            let cmds = format!("{}", &cmd_cargo);
            let (_code, output, error) = run_script::run(&cmds, &args, &options).unwrap();
            println!("{}", output);
            dbg!(error);
        }
    }
}

//const RUM_CMD_BAT: &str = r#"bat $RS_FILE"#;

// https://doc.rust-lang.org/rust-by-example/std_misc/arg/matching.html
// https://doc.rust-lang.org/rust-by-example/flow_control/match.html

// 题外话
// https://doc.rust-lang.org/rust-by-example/flow_control/match.html
// https://stackoverflow.com/questions/25383488/how-to-match-a-string-against-string-literals-in-rust
