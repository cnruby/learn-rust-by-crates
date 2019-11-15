// File ./exmaples/use_prettytable.rs
// clear && cargo run --example use_prettytable | bat -l cmd

#[macro_use]
extern crate prettytable;

fn main() {
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
}
