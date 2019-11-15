// Goto the Project "bin-local-hello"
// cargo run --example usage -- prettytable

/*
// an `extern crate` loading macros must be at the crate root
#[macro_use]
extern crate prettytable;
*/

pub fn use_prettytable() {
    // ANCHOR: use_prettytable
    // File: lib-hello/src/other/crate_tools/mod.rs
    // Function use_prettytable()

    let instance = "Hello";

    let copy_instance = instance;

    let table = table!(
        ["Name", "Value", "Remark"],
        [
            "instance reference address",
            format!("{:p}", instance),
            "is equal to the following line"
        ],
        [
            "copy_instance reference address",
            format!("{:p}", copy_instance),
            ""
        ],
        [
            "instance address",
            format!("{:p}", &instance),
            "is not equal to the following line"
        ],
        ["copy_instance address", format!("{:p}", &copy_instance), ""]
    );

    table.printstd();

    // ANCHOR_END: use_prettytable
}
