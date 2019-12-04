

pub const MOVE_VEC_OK :&str = r#"#![allow(unused_variables)]
pub fn main() {
    let instance = vec![1, 2, 3];
    println!("The variable instance before borrowing: {:?}", instance);
    let ref_instance = &instance;
    let equal_to_val = move |input_var| input_var == ref_instance;
    println!("The variable instance after borrowing: {:?}", instance);
    let input_instance = vec![1, 2, 3];
    assert!(equal_to_val(&input_instance));
}
"#;

pub const MOVE_VEC_ERR_03 :&str = r#"#![allow(unused_variables)]
pub fn main() {
    let instance = vec![1, 2, 3];
    println!("The variable instance before borrowing: {:?}", instance);
    let equal_to_val = move |input_var| input_var == &instance;
    println!("The variable instance after borrowing: {:?}", instance);
    let input_instance = vec![1, 2, 3];
    assert!(equal_to_val(&input_instance));
}
"#;

pub const MOVE_VEC_ERR_04 :&str = r#"#![allow(unused_variables)]
pub fn main() {
    let instance = vec![1, 2, 3];
    println!("The variable instance before borrowing: {:?}", instance);
    let equal_to_val = move |input_var| input_var == instance;
    println!("The variable instance after borrowing: {:?}", instance);
    let input_instance = vec![1, 2, 3];
    assert!(equal_to_val(input_instance));
}
"#;