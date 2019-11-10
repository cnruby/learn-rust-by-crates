pub const CLOSURE_MOVE_VEC_OK :&str = r#"// File: ./examples/closure_move_vec.rs

#![allow(unused_variables)]

fn main() {
    let instance = vec![1, 2, 3];

    println!("The variable instance before borrowing: {:?}", instance);

    let ref_instance = &instance;
    let equal_to_val = move |input_var| input_var == ref_instance;

    println!("The variable instance after borrowing: {:?}", instance);

    let input_instance = vec![1, 2, 3];
    assert!(equal_to_val(&input_instance));
}

#[cfg(feature = "cp")]
fn main() {
    let instance = vec![1, 2, 3];
    println!("The variable instance before borrowing: {:?}", instance);

    let equal_to_val = move |input_var| input_var == &instance;

    println!("The variable instance after borrowing: {:?}", instance);

    let input_instance = vec![1, 2, 3];
    assert!(equal_to_val(&input_instance));
}
"#;

pub const CLOSURE_MOVE_VEC_ERR :&str = r#"// File: ./examples/closure_move_vec.rs

#![allow(unused_variables)]

fn main() {
    let instance = vec![1, 2, 3];

    println!("The variable instance before borrowing: {:?}", instance);
    let equal_to_val = move |input_var| input_var == instance;

    println!("The variable instance after borrowing: {:?}", instance);

    let input_instance = vec![1, 2, 3];
    assert!(equal_to_val(input_instance));
}

#[cfg(all(not(feature = "ok"), not(feature = "err"), not(feature = "cp") ))]
fn main() {
    use aide::hello;
    hello();
}
"#;
