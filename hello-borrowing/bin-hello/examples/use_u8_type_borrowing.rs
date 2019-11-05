// File: ./examples/use_u8_type_borrowing.rs
// clear && cargo run --example use_u8_type_borrowing

fn main() {
    let instance = 42u8;
    println!("{}", instance);
    println!("{:p}", &instance);

    let copy_instance = instance;
    println!("{:p}", &copy_instance);

    println!("{}", instance);
}
