// File: ./bin-hello/examples/for_loop/for_next.rs
// clear && cargo run --example for_loop --features ok -- for_next | bat -l cmd
// clear && cargo run --example for_loop --features err_04
// clear && cargo run --example for_loop --features err_05
// clear && cargo run --example for_loop -- for_next

//=======


//=======
#[cfg(feature = "ok")]
pub fn adjoin() {
    // ANCHOR: feature-ok
    // File: ./bin-hello/examples/for_loop/for_next.rs
    // #[cfg(feature = "ok")]

    let mut instance = vec![33u8; 5];
    let mut iter = instance.iter_mut();
    
    loop {
        match iter.next() {
            Some(x) => println!("{:?}", x),
            None => break,
        }
    }
    
    println!("{:?}", instance);

    // ANCHOR_END: feature-ok
}



//=======
#[cfg(feature = "cp")]
pub fn adjoin() {
    // ANCHOR: feature-cp
    // File: ./bin-hello/examples/for_loop/for_next.rs
    // #[cfg(feature = "cp")]

    let mut instance = vec![33u8; 5];
    
    for index in 0..instance.len() {
        instance[index] = (index as u8) + instance[index];
        println!("{:?}", instance[index]);
    }

    println!("{:?}", instance);

    // ANCHOR_END: feature-cp
}




//=======
#[cfg(feature = "okey")]
pub fn adjoin() {
    // ANCHOR: feature-okey
    // File: ./bin-hello/examples/for_loop/for_next.rs
    // #[cfg(feature = "okey")]

    let mut instance = vec![33u8; 5];
    
    for (index, item) in instance.iter_mut().enumerate() {
        *item = (index as u8) + *item;
        println!("{:?}", *item);
    }

    println!("{:?}", instance);

    // ANCHOR_END: feature-okey
}




//=======
#[cfg(feature = "okay")]
pub fn adjoin() {
    // ANCHOR: feature-okay
    // File: ./bin-hello/examples/for_loop/for_next.rs
    // #[cfg(feature = "okay")]

    let instance: Vec<_> = vec![33u8; 5]
        .into_iter()
        .enumerate()
        .map(|(index, item)| (index as u8) + item)
        .collect();
    
    println!("{:?}", instance);

    // ANCHOR_END: feature-okay
}




//=======
#[cfg(feature = "err_07")]
pub fn adjoin() {
    // ANCHOR: feature-error_01
    // File: ./bin-hello/examples/for_loop/for_next.rs
    // #[cfg(feature = "err_07")]

    let mut instance = vec![33u8; 5];
    //let mut iter = instance.iter_mut();   // OK
    let iter = instance.iter_mut();
    
    loop {
        match iter.next() {
            Some(x) => println!("{:?}", x),
            None => break,
        }
    }
    
    println!("{:?}", instance);
    // ANCHOR_END: feature-error_01
}




//=======
#[cfg(feature = "err_08")]
pub fn adjoin() {
    // ANCHOR: feature-error_02
    // File: ./bin-hello/examples/for_loop/for_next.rs
    // #[cfg(feature = "err_08")]

    let instance = vec![33u8; 5];
    //let mut iter = IntoIterator::into_iter(instance);
    let mut iter = instance.into_iter();
    
    loop {
        match iter.next() {
            Some(x) => println!("{:?}", x),
            None => break,
        }
    }
    
    println!("{:?}", instance);
    
    // ANCHOR_END: feature-error_02
}



//=======
#[cfg(all(
    not(feature = "ok"), 
    not(feature = "okay"),
    not(feature = "okey"),
    not(feature = "cp"),
    not(feature = "err_07"),
    not(feature = "err_08"),
))]
pub fn adjoin() {
    use aide::hello;
    hello();
}

// https://www.reddit.com/r/rust/comments/61x2yd/idiomatic_way_to_handle_modifying_vectors_in_a/