// File: ./bin-hello/examples/for_loop/for_into_iter.rs
// clear && cargo run --example for_loop --features ok -- for_into_iter | bat -l cmd
// clear && cargo run --example for_loop --features okay -- for_into_iter | bat -l cmd
// clear && cargo run --example for_loop --features err_04
// clear && cargo run --example for_loop --features err_05
// clear && cargo run --example for_loop --features err_06
// clear && cargo run --example for_loop -- for_into_iter

#[cfg(feature = "ok")]
pub fn adjoin() {
    // ANCHOR: feature-ok
    // File: ./bin-hello/examples/for_loop/for_into_iter.rs
    // #[cfg(feature = "ok")]

    let mut instance = vec![33u8, 42];
    println!("1. instance = {:?}", instance);

    for item in instance.iter() {
        //item = 2;
        println!("item = {:p}", item);
    }
    println!("2. instance = {:?}", instance);

    let ref_instance = &mut instance;
    println!("1. ref_instance = {:p}", ref_instance);
    for item in ref_instance.into_iter() {
        *item = 3;
        println!("item = {:p}", item);
    }
    println!("3. instance = {:?}", instance);

    let ref_instance = &mut *instance;
    println!("1. ref_instance = {:p}", ref_instance);
    for item in ref_instance.into_iter() {
        *item = 4;
        println!("item = {:p}", item);
    }
    println!("4. instance = {:?}", instance);

    for item in instance.iter_mut() {
        *item = 5;
        println!("item = {:p}", item);
    }
    println!("5. instance = {:?}", instance);

    for item in instance.into_iter() {
        //item = 6;
        println!("item = {:p}", &item);
    }
    //println!("6. instance = {:?}", instance);

    // ANCHOR_END: feature-ok
}




#[cfg(feature = "cp")]
pub fn adjoin() {
    // ANCHOR: feature-cp
    // File: ./bin-hello/examples/for_loop/for_into_iter.rs
    // #[cfg(feature = "cp")]

    let mut instance = vec![33u8, 42];
    println!("1. instance = {:?}", instance);

    let ref_instance :&mut Vec<u8> = &mut instance;
    println!("1. ref_instance = {:?}", ref_instance);

    for item in ref_instance.into_iter() {
        *item = 1;
    }

    println!("2. ref_instance = {:?}", ref_instance);
    println!("2. instance = {:?}", instance);

    // ANCHOR_END: feature-cp
}





#[cfg(feature = "okey")]
pub fn adjoin() {
    // ANCHOR: feature-okey
    // File: ./bin-hello/examples/for_loop/for_into_iter.rs
    // #[cfg(feature = "okey")]

    let mut instance = vec![33u8, 42];
    println!("1. instance = {:?}", instance);
    println!("1. &instance = {:p}", &instance);

    let ref_instance :&mut [u8] = &mut instance;
    //let ref_instance = &mut *instance;
    //let ref_instance :&mut [u8] = &mut *instance;
    println!("1. ref_instance = {:p}", ref_instance);
    for item in ref_instance.into_iter() {        
        *item = 1;
        println!("item = {:p}", item);
    }

    println!("2. ref_instance = {:p}", ref_instance);
    println!("2. instance = {:?}", instance);

    // ANCHOR_END: feature-okey
}





#[cfg(feature = "okay")]
pub fn adjoin() {
    // Compare to "err_09"
    // ANCHOR: feature-okay
    // File: ./bin-hello/examples/for_loop/for_into_iter.rs
    // #[cfg(feature = "okay")]

    let instance = vec![33u8, 42];

    for item in instance.into_iter() {
        //println!("item = {:p}", item);
        println!("item = {:p}", &item);
    }

    // ANCHOR_END: feature-okay

    //println!("instance = {:?}", instance); // ERROR
}





#[cfg(feature = "err_09")]
pub fn adjoin() {
    // ANCHOR: feature-error_01
    // File: ./bin-hello/examples/for_loop/for_iter_mut.rs
    // #[cfg(feature = "err_09")]

    let instance = vec![33u8, 42];

    for item in instance.into_iter() {
        //item = 1;
        println!("item = {:p}", &item);
    }

    println!("instance = {:?}", instance);

    // ANCHOR_END: feature-error_01
}



#[cfg(all(
    not(feature = "ok"), 
    not(feature = "okay"),
    not(feature = "okey"),
    not(feature = "cp"),
    not(feature = "err_09"),
))]
pub fn adjoin() {
    use aide::hello;
    hello();
}