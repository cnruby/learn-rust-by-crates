# 应用篋：宏`dbg!`与可变实例

## 学习内容
- 了解和学习使用宏方法`dbg!`的借用实例

## 篇目

- [宏`dbg!`与借用机制](#宏dbg与借用机制)
- [可变对象及其固定引用对象](#可变对象及其固定引用对象)
- [可变对象及其可变引用对象](#可变对象及其可变引用对象)
- [题外话](#题外话)
- [创建类型`String`对象方法](#创建类型`String`对象方法)
- [类型`String`：方法`push`和`push_str`](#类型string方法push和push_str)
- [参考资料](#参考资料)

## 宏`dbg!`与借用机制

　　宏方法`dbg!`实现打印并返回给定表达式的值，目的进行快速而简陋的调试。该宏类似于程序输出宏`println!`，但是宏方法`dbg!`使用简单方便，并且它不是以与程序输出在一起，而是单独输出。所以，当使用在线图书执行它时，我们将看不到其输出结果。

　　先看一个宏方法`dbg!`的简单应用实例：

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/dbg/mut_macro.rs:feature-cp }}
```

其输出结果如下：

```rust,no_run,noplaypen
```

　　[官方文档](id_01)解释宏方法`dbg!`如下：

> Invoking the macro on an expression moves and takes ownership of it before returning the evaluated expression unchanged. If the type of the expression does not implement Copy and you don't want to give up ownership, you can instead borrow with dbg!(&expr) for some expression expr.

中文翻译：
> 调用基于表达式`expr`的宏，会转移对象并获取其所有权，然后再返回已求值的表达式不变。如果表达式的类型未实现特质`Copy`且不想放弃所有权，则对于一些表达式`expr`可以改用这种调用方法`dbg!(＆expr)`。

　　下面程序的目的就是来验证上面阐述的第一句话。一旦使用了没有实现特质`Copy`的类型`String`对象，是否宏方法`dbg!`转移该对象，并且调用该宏结束以后，就拿走了该对象的所有权。只要运行一下该程序，就得到了验证。在实际中，都会使用这种`dbg!(＆expr)`调用方法。

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/dbg/mut_macro.rs:feature-error_01 }}
```

## 可变对象及其固定引用对象

　　对于可变类型对象，在使用宏方法`dbg!`时，可以绑定其固定引用对象，具体实例如下：

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/dbg/mut_macro.rs:feature-ok }}
```

　　上面程序运行正常，说明固定引用对象`ref_string`的借用是正常的。那么我们可否绑定其可变引用对象呢？

## 可变对象及其可变引用对象

　　下面程序将回答上面的问题。对于可变类型对象，在使用宏方法`dbg!`时，可以绑定其可变引用对象，将会发生什么？具体实例如下：

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/dbg/mut_macro.rs:feature-error_03 }}
```

　　对于可变引用对象，宏方法`dbg!`也将其可变对象的所有权收回了。

## 题外话

### 创建类型`String`对象方法

```rust
let new_string :String = String::new();
let from_string :String = String::from("Hello");
let format_string :String = format!("{}", "Hello");
let to_string :String = "Hello".to_string();
```

### 类型`String`：方法`push`和`push_str`

　　在创建类型`String`的可变对象时，经常会使用到方法`push()`和`push_str()`，前者方法参数类型是`char`，而后者方法参数类型`&str`。

## 参考资料
- [std macro.dbg](https://doc.rust-lang.org/std/macro.dbg.html)
- [rusts-dbg-macro](https://brown121407.github.io/programming/2019/01/18/rusts-dbg-macro.html)
- [does-rust-have-a-debug-macro](https://stackoverflow.com/questions/38141056/does-rust-have-a-debug-macro)
- [Generator size: borrowed variables are...](https://github.com/rust-lang/rust/issues/59087)

[id_01]:https://doc.rust-lang.org/std/macro.dbg.html