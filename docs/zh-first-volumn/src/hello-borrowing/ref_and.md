# 题外话：关键词`ref`与引用符`&`

　　在下面一段代码只有一个`let`语句里，目的是说明该`＆ref`操作等价于什么也没有做，它们是相反的操作，类似于操作`*`和`&`也是相反的操作。

```rust
{{ #include ../../../../hello-borrowing/lib-hello/src/immut/type_ref/mod.rs:use_ref_and }}
```

　　借助于工具cargo-clippy，使用下面命令：

```bash
# 工具cargo-clippy命令
cargo　clippy
```

可以获取如下编译器的警告信息：

```bash
warning: `ref` on an entire `let` pattern is discouraged, take a reference with `&` instead
   --> lib-hello/src/immut/type_ref/mod.rs:105:9
    |
105 |     let ref y: u8 = x;
    |     ----^^^^^--------- help: try: `let y: &u8 = &x;`
    |
    = note: `#[warn(clippy::toplevel_ref_arg)]` on by default
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#toplevel_ref_arg
```

　　从上面警告，可以解读这些内容：

- 在上面程序一段代码包括四个`let`语句里，中间两个是等价的；
- 不鼓励在整个`let`语句上使用关键词引用符`ref`，而应使用借用符`＆`；
- 从中可以知道，关键词`ref`不应该在`let`语句上使用，而更适合在`macth`语句里使用它。借用符`＆`要引用一个对象，而`ref`通过引用而不是按值绑定到位置。换句话说，借用符`＆`实现简单的借用，而引用符`ref`则是“借用我，到与我相匹配的位置上去”；

　　在宏`println!()`打印内存地址信息时，只有非引用对象变量，才在变量前面使用借用符`&`。要是去掉该引用符`&`，编译器将会出现下面错误信息，从中也可以知道，谁是引用谁不是引用对象：

```bash
error[E0277]: the trait bound `u8: std::fmt::Pointer` is not satisfied
 --> src/main.rs:5:26
  |
5 |     println!("x = {:p}", x);
  |                          ^ the trait `std::fmt::Pointer` is not implemented for `u8`
  |
  = note: required by `std::fmt::Pointer::fmt`
```

## 参考资料
- [Ref keyword versus &](https://users.rust-lang.org/t/ref-keyword-versus/18818/4)
- [The ref pattern](https://doc.rust-lang.org/rust-by-example/scope/borrow/ref.html)
- [& vs. ref in Rust patterns](http://xion.io/post/code/rust-patterns-ref.html)
- [ref-patterns-destructuring-and-invisible-borrows](https://medium.com/@robertgrosse/ref-patterns-destructuring-and-invisible-borrows-2f8ae6902656)