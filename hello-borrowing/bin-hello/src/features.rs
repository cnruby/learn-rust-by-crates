use run_script::ScriptOptions;

pub fn match_mode(args: &Vec<String>, mode: &str) {
    let options = ScriptOptions::new();
    let file_name = &args[2];
    let rs_file: &str = &format!("RS_FILE=./examples/{}.rs", file_name);
    let cmd_cargo: &str = &format!("cargo run --example {} --features '{}'", file_name, mode);
    //println!("This is the answer: `{}`", cmd_cargo);
    //println!("This is the answer: {}!", mode);

    run_cmd(
        &args,
        &options,
        &format!(
            "{}\n{}\n{}",
            rs_file, cmd_cargo, RUM_CMD_BAT
        ),
    );
}

const RUM_CMD_BAT: &str = r#"bat $RS_FILE"#;

fn run_cmd(args: &Vec<String>, options: &ScriptOptions, cmd_str: &str) {
    //println!("This is the answer!");
    let (_code, output, error) = run_script::run(cmd_str, &args, &options).unwrap();

    //println!("Exit Code: {}\n\n", code);
    //println!("Tool bat: Output Codes >>>>>>>>>>>>>> :\n\n{}\n\n", output);
    println!("{}\n\n", output);
    println!("Compiler: Output Info  >>>>>>>>>>>>>> :\n\n{}", error);
}
// https://doc.rust-lang.org/rust-by-example/std_misc/arg/matching.html