// Goto the Project "bin-local-hello"
// cargo run --example usage -- u8_type
// cargo run --example usage -- str_type
// cargo run --example usage -- references_simple
// cargo run --example usage -- ref_and

#![allow(unused_variables)]

pub fn use_u8_type() {
    // ANCHOR: use_u8_type
    // File: lib-hello/src/immut/type_ref/mod.rs
    // Function use_u8_type()

    let instance = 42u8;
    println!("{}", instance);
    println!("{:p}", &instance);

    let copy_instance = instance;
    println!("{:p}", &copy_instance);

    println!("{}", instance);
    
    // ANCHOR_END: use_u8_type
}



pub fn use_references_simple() {
    // ANCHOR: use_references_simple
    // File: lib-hello/src/immut/type_ref/mod.rs
    // Function use_references_simple()

    let instance: &str = "Hello";
    let instance = "Hello";

    let copy_instance: &str = instance;
    let copy_instance = instance;

    println!("instance reference address = {:p}", instance);
    println!("copy_instance reference address = {:p}", copy_instance);

    println!("instance address = {:p}", &instance);
    println!("copy_instance address = {:p}", &copy_instance);

    // ANCHOR_END: use_references_simple
}




pub fn use_str_type() {
    // ANCHOR: use_str_type
    // File: lib-hello/src/immut/type_ref/mod.rs
    // Function use_str_type()

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

    // ANCHOR_END: use_str_type
}



pub fn use_ref_and() {
    // ANCHOR: use_ref_and
    // File: lib-hello/src/immut/type_ref/mod.rs
    // Function use_ref_and()

    let x: u8 = 33;
    let w: &u8 = &x;
    let ref y: u8 = x;
    let z: &u8 = y;
    
    println!("x = {:p}", &x);
    println!("w = {:p}", w);
    println!("y = {:p}", y);
    println!("z = {:p}", z);

    println!("x = {}", x);
    println!("w = {}", w);
    println!("y = {}", y);
    println!("z = {}", z);

    let &ref y  = &x;
    println!("y = {:p}", y);
    println!("y = {}", y);
    println!();
    
    let x: Vec<u8> = vec![33, 42];
    let w: &Vec<u8> = &x;
    let ref y: Vec<u8> = x;
    let z: &Vec<u8> = y;
    
    println!("x = {:p}", &x);
    println!("w = {:p}", w);
    println!("y = {:p}", y);
    println!("z = {:p}", z);

    println!("x = {:?}", x);
    println!("w = {:?}", w);
    println!("y = {:?}", y);
    println!("z = {:?}", z);

    let &ref y  = &x;
    println!("y = {:p}", y);
    println!("y = {:?}", y);

    // ANCHOR_END: use_ref_and
}

