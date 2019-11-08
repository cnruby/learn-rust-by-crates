# 应用篋：方法借用实例

## 学习内容
- 了解和学习Rust语言方法借用实例

## 篇目

- [实现复制特质`Copy`类型的借用实例](#实现复制特质copy类型的借用实例)
- [未实现复制特质`Copy`类型的借用实例](#未实现复制特质copy类型的借用实例)
- [借用机制代码实例](#借用机制代码实例)
- [题外话](#题外话)
- [Rust语言下横杆`_`](#Rust语言下横杆_)
- [向量类型完整定义方法](#向量类型完整定义方法)
- [向量宏vec!](#向量宏vec)
- [参考资料](#参考资料)

## 实现复制特质`Copy`类型的借用实例

　　在下面程序方法`main()`的三段代码里，表面上并没看到复制变量`num`，但是实际上存在变量`num`的复制。一旦调用方法`fn_borrow()`，方法内部就进行了复制特质`Copy`的复制。这是怎么知道的呢？

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/use_kw_fn_u8.rs:features-cp }}
```

　　下面程序代码，说明了上面问题。该程序在上面程序基础上，把变量`num`在调用方法之前之后以及在方法内的内存地址都打印出来。

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/use_kw_fn_u8.rs:features-ok }}
```

　　这是上面程序输出结果。从结果可以看到，在调用方法之前之后变量`num`内存地址是完全一样的，只是在方法内变量`num`内存地址是不一样的。这说明方法内部从一开始就复制了变量`num`，这是因为类型u8复制特质`Copy，所以可以存在这样借用机制。

```bash
[bin-hello/examples/use_kw_fn_u8.rs:17] num = 42
───────┬─────────────────────────────────────────────────────────────────────────
       │ STDIN
───────┼─────────────────────────────────────────────────────────────────────────
   1   │ Before fn = 0x7fff531fd04f
   2   │ inside fn = 0x7fff531fcf57
   3   │ After fn = 0x7fff531fd04f
───────┴─────────────────────────────────────────────────────────────────────────
```

## 未实现复制特质`Copy`类型的借用实例

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/kw_fn_vec_u8.rs:feature-err }}
```

　　在下面的程序里，在执行该程序以后，可以看到错误提示信息，这是因为类型`Vec<u8>`没有实现复制特质`Copy`，而方法`fn_borrow`里面还是执行了变量的复制，相当于执行了下面一行代码：

```rust,no_run,noplaypen
let vec_u8s = vec_instance;
```

　　这是上面程序输出错误结果：

```bash
error[E0382]: borrow of moved value: `vec_instance`
  --> bin-hello/examples/kw_fn_vec_u8.rs:32:22
   |
29 |     let vec_instance = Vec::new();
   |         ------------ move occurs because `vec_instance` has type `std::vec::Vec<u8>`, which does not implement the `Copy` trait
30 |     println!("{:p}", &vec_instance);
31 |     fn_borrow(vec_instance);
   |               ------------ value moved here
32 |     println!("{:p}", &vec_instance);
   |                      ^^^^^^^^^^^^^ value borrowed here after move
```

　　上面程序类型`Vec<u8>`是可以实现复制特质`Copy`，但是怎么样可以简单这种功能实现呢？

## 借用机制代码实例

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/kw_fn_vec_u8.rs:feature-ok }}
```

　　上面程序代码，使用了Rust语言的借用机制，实现了变量的借用，以达到传递变量值到函数或者方法。在Rust语言内部，存在一行下面代码，通过传递类型Vec<u8>的引用对象`＆vec_instance`到方法`fn_borrow()`，以实现这种借用机制：

```rust,no_run,noplaypen
let vec_u8s = ＆vec_instance;
```

## 题外话
### Rust语言下横杆`_`

　　在上面程序中，有两个地方存在下横杆`_`，这是Rust语言待定符号。比如，下面代码，要是函数或者方法参数还没有确定如何使用，就可以使用这个待定符号下横杆`_`。

```rust,no_run,noplaypen
fn fn_borrow(_: u8) {}
```

　　下面是另外一个代码实例，我们知道，接下来该变量会存在类型定义，如作为方法参数使用该变量，也可以使用这个待定符号下横杆`_`。但是要是没有作为方法参数和其他使用该变量，编译器就会报错。

```rust,no_run,noplaypen
let vec_instance :Vec<_> = vec![33, 42];
```

### 向量类型完整定义方法

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/kw_fn_vec_u8.rs:feature-cp }}
```

　　上面程序方法｀main()｀第二段的第一行代码，是类型向量最完整的表达形式，它也可以简写为下面这种常见的一行代码。同时也要注意到，前面代码里方法参数也是简写形式，而上面程序代码也是完整形式。这样定义对象变量的类型`Vec::<u8>`形式，与方法参数定义的引用类型`&Vec::<u8>`形式具有其一致性。

```rust,no_run,noplaypen
let mut vec_instance = Vec::new();
```

### 向量宏vec!

　　在前面程序代码里，可以看到类型向量宏vec!，这是创建其对象的宏，它类似于打印宏`println!`。上面一行宏vec!代码，相当于下面的三行代码。当然其中不同的是，下面类型向量对象是可变绑定方式，而前面的向量对象是不可变绑定方式。Rust语言为我们创建类型向量对象提供了两种不同的方式：不可变和可变的向量绑定对象。

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/kw_fn_vec_u8.rs:feature-cp-vec }}
```

　　其实，类型字符串也有同样的功能，只是表达方式不同而已：

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/use_string_mut_new.rs }}
```

## 参考资料
- [Rustdoc: No mention of copy trait for primitive types](https://github.com/rust-lang/rust/issues/25893)
- [Understanding Rust: ownership, borrowing, lifetimes](https://medium.com/@bugaevc/understanding-rust-ownership-borrowing-lifetimes-ff9ee9f79a9c)