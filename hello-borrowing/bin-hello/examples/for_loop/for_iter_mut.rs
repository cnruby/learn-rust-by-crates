// File: ./bin-hello/examples/for_loop/for_iter_mut.rs
// clear && cargo run --example for_loop --features ok -- for_iter_mut | bat -l cmd
// clear && cargo run --example for_loop --features err_04
// clear && cargo run --example for_loop --features err_05
// clear && cargo run --example for_loop -- for_iter_mut

#[cfg(feature = "ok")]
pub fn adjoin() {
    // ANCHOR: feature-ok
    // File: ./bin-hello/examples/for_loop/for_iter_mut.rs
    // #[cfg(feature = "ok")]

    let mut instance = vec![33u8, 42];
    println!("1. instance = {:?}", instance);
    println!("1. &instance = {:p}", &instance);

    for item in instance.iter_mut() {
        *item = 1;
        println!("item = {:p}", item);
    }

    println!("2. instance = {:?}", instance);
    println!("２. &instance = {:p}", &instance);

    // ANCHOR_END: feature-ok
}




#[cfg(feature = "cp")]
pub fn adjoin() {
    // ANCHOR: feature-cp
    // File: ./bin-hello/examples/for_loop/for_iter_mut.rs
    // #[cfg(feature = "cp")]

    let mut instance = vec![33u8, 42];
    println!("1. instance = {:?}", instance);
    println!("1. &instance = {:p}", &instance);

    let ref_instance = &mut instance;
    println!("1. ref_instance = {:?}", ref_instance);
    println!("1. ref_instance = {:p}", ref_instance);

    for item in ref_instance.iter_mut() {
        *item = 1;
        println!("item = {:p}", item);
    }

    println!("2. ref_instance = {:?}", ref_instance);
    println!("2. ref_instance = {:p}", ref_instance);
    println!("2. instance = {:?}", instance);

    // ANCHOR_END: feature-cp
}





#[cfg(feature = "okey")]
pub fn adjoin() {
    // ANCHOR: feature-okey
    // File: ./bin-hello/examples/for_loop/for_iter_mut.rs
    // #[cfg(feature = "okey")]

    let mut instance = vec![33u8, 42];
    println!("1. instance = {:?}", instance);
    println!("1. &instance = {:p}", &instance);

    let ref_instance = &mut *instance;
    println!("1. ref_instance = {:p}", ref_instance);
    for item in ref_instance.iter_mut() {        
        *item = 1;
        println!("item = {:p}", item);
    }

    println!("2. ref_instance = {:p}", ref_instance);
    println!("2. instance = {:?}", instance);

    // ANCHOR_END: feature-okey
}





#[cfg(feature = "okay")]
pub fn adjoin() {
    // ANCHOR: feature-okay
    // File: ./bin-hello/examples/for_loop/for_iter_mut.rs
    // #[cfg(feature = "okay")]

    let mut instance = vec![33u8, 42];
    println!("1. instance = {:?}", instance);
    println!("1. &instance = {:p}", &instance);

    let ref_instance = &mut instance;
    println!("1. ref_instance = {:?}", ref_instance);
    println!("1. ref_instance = {:p}", ref_instance);

    for item in ref_instance.iter_mut() {
        *item = 1;
        println!("item = {:p}", item);
    }
    println!("2. ref_instance = {:?}", ref_instance);
    println!("2. ref_instance = {:p}", ref_instance);

    let ref_instance = &mut *instance;
    println!("3. ref_instance = {:p}", ref_instance);
    for item in ref_instance.iter_mut() {        
        *item = 1;
        println!("item = {:p}", item);
    }

    println!("3. ref_instance = {:?}", ref_instance);
    println!("3. ref_instance = {:p}", ref_instance);
    println!("2. instance = {:?}", instance);


    // ANCHOR_END: feature-okay
}




#[cfg(feature = "err_04")]
pub fn adjoin() {
    // ANCHOR: feature-error_01
    // File: ./bin-hello/examples/for_loop/for_iter_mut.rs
    // #[cfg(feature = "err_04")]

    let mut instance = vec![33u8, 42];
    let ref_instance = &mut instance;

    for item in ref_instance {
        *item = 1;
    }

    println!("{:?}", ref_instance); // ERROR
    println!("{:?}", instance);

    // ANCHOR_END: feature-error_01
}




#[cfg(feature = "err_05")]
pub fn adjoin() {
    // ANCHOR: feature-error_02
    // File: ./bin-hello/examples/for_loop/for_iter_mut.rs
    // #[cfg(feature = "err_05")]

    let mut instance = vec![33u8, 42];
    let ref_instance = &mut *instance;

    for item in ref_instance {
        *item = 1;
    }
    
    println!("{:?}", ref_instance); //ERROR
    println!("{:?}", instance);

    // ANCHOR_END: feature-error_02
}



#[cfg(feature = "err_06")]
pub fn adjoin() {
    // ANCHOR: feature-error_03
    // File: ./bin-hello/examples/for_loop/for_iter_mut.rs
    // #[cfg(feature = "err_06")]

    let mut instance = vec![33u8, 42];
    println!("1. instance = {:?}", instance);

    for item in instance.into_iter() {
        item = 1;
    }

    println!("2. instance = {:?}", instance);

    // ANCHOR_END: feature-err_06
}




#[cfg(all(
    not(feature = "ok"), 
    not(feature = "okay"),
    not(feature = "okey"),
    not(feature = "cp"),
    not(feature = "err_04"),
    not(feature = "err_05"),
    not(feature = "err_06"),
))]
pub fn adjoin() {
    use aide::hello;
    hello();
}