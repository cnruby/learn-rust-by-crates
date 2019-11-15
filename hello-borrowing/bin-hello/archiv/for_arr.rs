// File: ./examples/for_arr.rs
// clear && cargo run --example for_arr --features ok | bat -l cmd
// clear && cargo run --example for_arr --features err

#[cfg(feature = "ok")]
// ANCHOR: feature-ok
// File: ./examples/for_arr.rs
fn main() {
    let instance = [1u8, 2, 3];
    println!("{:>17} {} {:p}", "instance ref", "=", &instance);

    print!("{:>17} {} ", "for ref", "=");
    for item in &instance {
        let ref_item :&u8 = item;
        print!("{:p} ", ref_item);
    }
    println!("");

    print!("{:>17} {} ", "for .iter()", "=");
    for item in instance.iter() {
        let ref_item :&u8 = item;
        print!("{:p} ", ref_item);
    }
    println!("");

    print!("{:>17} {} ", "for .into_iter()", "=");
    for item in instance.into_iter() {
        //let u8_item :u8 = item;  // ERROR: item IS &u8!!!
        let ref_item :&u8 = item;
        print!("{:p} ", ref_item);
    }
    println!("");

    println!("{:>17} {} {:?}", "instance arr", "=", instance);
}
// ANCHOR_END: feature-ok

#[cfg(feature = "err")]
// error[E0277]
// ANCHOR: feature-err
fn main() {
    let instance = [1u8, 2, 3];
    
    for item in instance {
        print!("{:p} ", item);
    }
    
    println!("instance array = {:?}", instance);
}
// ANCHOR_END: feature-err

#[cfg(all(not(feature = "ok"), not(feature = "err") ))]
fn main() {
    use aide::hello;
    hello();
}
