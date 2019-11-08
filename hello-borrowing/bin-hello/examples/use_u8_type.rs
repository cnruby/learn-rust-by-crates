// File: ./examples/use_u8_type.rs
// clear && cargo run --example use_u8_type

fn main() {
    let instance = 42u8;
    println!("{}", instance);
    println!("{:p}", &instance);

    let copy_instance = instance;
    println!("{:p}", &copy_instance);

    println!("{}", instance);
}
