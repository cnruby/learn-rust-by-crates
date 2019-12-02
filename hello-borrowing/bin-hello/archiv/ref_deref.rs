#![allow(unused_variables)]

fn main () {
    let mut ch = '®';

    let ref_ch = &mut ch;
    println!("1. ref_ch = {:p}", ref_ch);
    println!("1. ref_ch = {:?}", ref_ch);

    // let deref_ch: &mut char = &mut ch;
    let deref_ch :*mut char = &mut ch;
    println!("deref_ch = {:?}", deref_ch);
    
    unsafe {
        println!("deref_ch = {:?}", *deref_ch);

        let ref_ch = &mut *deref_ch;
        println!("2. ref_ch = {:p}", ref_ch);
        println!("2. ref_ch = {:?}", ref_ch);
    }
}