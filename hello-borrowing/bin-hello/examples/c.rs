fn main() {
    let x = "你好";

    let x_1 = x.chars().nth(1);
    dbg!(x_1);

    let x_len = x.len();
    dbg!(x_len);

    let x_count = x.chars().count();
    dbg!(x_count);
}
