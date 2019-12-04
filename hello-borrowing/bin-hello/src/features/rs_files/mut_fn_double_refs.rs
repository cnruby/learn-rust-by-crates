

pub const DOUBLE_REFS_OK :&str = r#"#![allow(unused)]
pub fn main() {
    fn fn_borrowed(mut_ref: &mut i32) {}
    fn fn_borrow(mut_ref: &mut i32) {
        fn_borrowed(mut_ref); // mutable borrow occurs here
        let immut_ref = mut_ref; // immutable borrow occurs here
        println!("{}", immut_ref); // immutable borrow later used here
    }
}
"#;

pub const DOUBLE_REFS_ERR_01 :&str = r#"#![allow(unused)]
pub fn main() {
    fn fn_borrowed(mut_ref: &mut i32) {}
    fn fn_borrow(mut_ref: &mut i32) {
        let immut_ref = mut_ref;
        fn_borrowed(mut_ref);
        println!("{}", immut_ref);
    }
}
"#;

pub const DOUBLE_REFS_ERR_02 :&str = r#"#![allow(unused)]
pub fn main() {
    fn fn_borrowed(mut_ref: &mut i32) {}
    fn fn_borrow(mut_ref: &mut i32) {
        let immut_ref_ref = &mut_ref;
        fn_borrowed(mut_ref); 
        println!("{}", immut_ref_ref);
    }
}
"#;