fn main() {
    let a: [u8; 0] = [];
    dbg!(a);
    let b = a.clone();
    let c = a;
    dbg!(a, b, c);

    let a: [u8; 0] = [];
    dbg!(a);
    let b = a;
    let c = a.clone();
    dbg!(a, b, c);
}