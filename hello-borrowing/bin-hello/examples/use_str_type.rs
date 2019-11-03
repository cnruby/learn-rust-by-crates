#![allow(unused_variables)]

#[macro_use] extern crate prettytable;

fn main() {
    // move occurs because `instance` has type `&str`,
    // which does implement the `Copy` trait
    let instance = "hello";

    // The variable `instance` begin to move here
    let copy_instance = instance;
    // The variable `instance` moved here

    // The variable `instance` borrowed here after move
    println!("instance reference address = {:p}", instance);
    println!("copy_instance reference address = {:p}", copy_instance);

    println!("instance address      = {:p}", &instance);
    println!("copy_instance address = {:p}", &copy_instance);

    let table = table!(["Name", "Value", "Remark"],
                       ["instance reference address", format!("{:p}", instance), "is equal to the following line"],
                       ["copy_instance reference address", format!("{:p}", copy_instance), ""],
                       ["instance address", format!("{:p}", &instance), "is not equal to the following line"],
                       ["copy_instance address", format!("{:p}", &copy_instance), ""]);

    table.printstd();
}
