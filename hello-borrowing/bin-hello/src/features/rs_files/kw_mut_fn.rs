pub const KW_MUT_FN_OK: &str = r#"#![allow(unused)]

fn main() {
    fn bar(x: &mut i32) {}
    fn foo(a: &mut i32) {
        bar(a); // mutable borrow occurs here
        let ref y = a; // immutable borrow occurs here

        println!("{}", y); // immutable borrow later used here
    }
}
"#;

pub const KW_MUT_FN_ERR: &str = r#"#![allow(unused)]

fn main() {
    fn bar(x: &mut i32) {}
    fn foo(a: &mut i32) {
        let ref y = a; // a is borrowed as immutable.
        bar(a); // error: cannot borrow `*a` as mutable because
        println!("{}", y);
    }
}
"#;
