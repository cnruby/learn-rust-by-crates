fn main() {
    let x: u8 = 1;
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

    let x: Vec<u8> = vec![10, 20];
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
}
