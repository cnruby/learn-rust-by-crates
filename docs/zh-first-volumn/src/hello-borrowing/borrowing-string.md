# 应用篋：字符串类型借用方法

## 学习内容
- 了解和学习Rust语言类型`String`借用实例

## 篇目

- [变量生命周期](#变量生命周期)
- [错误使用变量实例](#错误使用变量实例)
- [借用机制代码实例](#借用机制代码实例)
- [题外话](#题外话)
- [开发工具`cargo-hack`](#开发工具cargo-hack)
- [参考资料](#参考资料)

## 变量生命周期

　　在官方文档英文解释中，无论是复制特质`Copy`，还是克隆特质`Clone`，都使用了动词`duplicate`，这明确说明了实际运作时，将再产生一份新对象，即：原对象和复制对象。要是没有达到这个目的，就不能称之为复制或者克隆。

```rust
let instance = String::from("Hello");
let copy_instance = instance;
```

　　在上面的代码实例里，尽管变量`instance`和`copy_instance`是不同的对象，且变量`copy_instance`是通过所谓”复制“方式产生的，但是它们不能同时使用。这种”复制“没有产生第二个对象，只是改变了对象名称及其内存地址不同而已，所以不是真正的复制功能。

　　下面示意图说明了，从第一行`let`语句开始到第二行`let`语句结束，内存数据储存的状态变化过程。

![image](../../hello-borrowing/images/hello_borrowing-10_string.png)

　　在第二个`let`语句结束以后，变量`instance`生命周期也就结束了，之后就不能再使用它了。这是因为类型`String`没有复制特质`Copy`的实现。要是一种类型实现了复制特质`Copy`，那么其生命周期还是存在的。

　　例如，类型正整数u8实现了复制特质`Copy`，使用这种隐式复制方法，该类型就生命周期还是存在的。哪些类型实现了复制特质`Copy`，请参考[Rust语言标准库文档](https://doc.rust-lang.org/std/marker/trait.Copy.html)。

　　类型正整数`u8`或者逻辑类型`bool`等都是实现了复制特质`Copy`的典型实例，而字符串`String`和向量Vec`等都是没有实现复制特质`Copy`的典型实例。

```rust
{{ #include ../../../../hello-borrowing/lib-hello/src/immut/type_ref/mod.rs:use_u8_type }}
```

## 错误使用变量实例

　　下面程序在方法`main()`有三段代码。第一段代码创建一个类型`String`对象`instance`；第二段代码是一种复制或者拷贝对象的行为，从上面变量生命周期可以知道，第三段代码的变量`instance`已经成为没有定义的变量。所以，编译器指出其错误，其含意如上所述。Rust语言使用了“移动（move）”，说明变量生命周期的过程。

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/string_type/string_str.rs:feature-err_01 }}
```

　　该程序输出结果如下：

```rust
error[E0382]: borrow of moved value: `instance`
  --> bin-hello/examples/string_type_str.rs:27:20
   |
19 |     let instance = String::from("hello");
   |         -------- move occurs because `instance` has type `std::string::String`, which does not implement the `Copy` trait
...
23 |     let copy_instance = instance;
   |                         -------- value moved here
...
27 |     println!("{}", instance);
   |                    ^^^^^^^^ value borrowed here after move
```

## 借用机制代码实例

　　下面看看Rust如何实现借用方法。这是典型Rust语言的代码。下面程序在方法`main()`有三段代码。在该方法里，类型`String`对象`instance`和`borrow_instance`始终是有效的。

```rust
{{ #include ../../../../hello-borrowing/bin-hello/examples/string_type/string_str.rs:feature-ok }}
```

　　与前面代码实例一样，第一段代码创建一个类型`String`对象`instance`；第二段代码也是一种复制或者拷贝对象的行为，但是该对象的类型是引用，这是Rust语言的一种借用机制，所谓”借用“，就是把对象`instance`的值借过来使用。第三段代码的变量`instance`和变量`borrow_instance`还都是可以使用的。

　　下面示意图告诉我们，程序代码对象的内存储存结构。

![image](../../hello-borrowing/images/hello_borrowing-11_string.png)

　　该程序输出结果如下，从这个结果可以看到两个内存是完全一样的，这个内存地址就是原始指针的地址。

```bash
───────┬────────────────────────────────────────────────────────────────────
       │ STDIN
───────┼────────────────────────────────────────────────────────────────────
   1   │ raw_instance = 0x7fca1bc03680
   2   │ borrow_instance = 0x7fca1bc03680
   3   │ Hello
   4   │ Hello
───────┴────────────────────────────────────────────────────────────────────
```

## 题外话

### 开发工具`cargo-hack`

　　开发工具`cargo-hack`解决工具`cargo`某些限制。工具`cargo-hack`，目前仅在Linux和macOS上进行过测试。在其他平台上可能无法正常工作。

```bash
cargo install cargo-hack
```

　　对于程序有属性`features`时，目前工具`cargo`需要对每一个属性`features`生成一个命令，而工具`cargo-hack`只要一个命令就可以运行所有属性`features`的程序。比如，上面程序`examples/string_type_str.rs`有两个属性`features`，这样就要两行不同的命令来执行程序：

```bash
clear && cargo run --example string_type_str --features ok | bat -l rs
clear && cargo run --example string_type_str --features err
```

而使用工具`cargo-hack`，只需要一行命令：

```bash
cargo hack check --example string_type_str --each-feature --no-dev-deps
```

## 参考资料
- [crate cargo-hack](https://crates.io/crates/cargo-hack)
- [trait std::marker::Copy](https://doc.rust-lang.org/std/marker/trait.Copy.html)