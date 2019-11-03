# 引用`Reference`与指针`Pointer`基本概念

## 学习内容
- 了解和学习引用`Reference`与指针`Pointer`概念

## 篇目

- [什么是指针`Pointer`](#什么是指针Pointer)
- [什么是引用`Reference`](#什么是引用Reference)
- [Rust语言的引用`Reference`与指针`Pointer`](#Rust语言的引用Reference与指针Pointer)
- [引用实例解释](#引用实例解释)
- [题外话](#题外话)
- [介绍Rust的指针类型](#介绍Rust的指针类型)
- [开发工具：软件篋prettytable-rs和ripgrep](#开发工具：软件篋prettytable-rs和ripgrep)
- [参考资料](#参考资料)

## 什么是指针`Pointer`

　　Rust语言技术术语，既有其本身专门的技术术语，如软件篋`crate`、借用`Borrowing`和生命周期`Lifttime`等，也有一般计算机科学技术术语，如变量、类型和指针`Pointer`等。但是，对于一般技术术语，在Rust语言里也是有不同的内涵意义和实现形式。

　　[引用英文][id_01]
> In computer science, a pointer is a programming language object that stores the memory address of another value located in computer memory. 

直接翻译：在计算机科学中，指针是编程语言的对象，用于存储位于计算机内存中的另一个值的内存地址。

解读：
- 指针是一个对象`object`或者说实例`instance`，它是指针类型的对象；
- 指针也是变量，与其它变量完全一样；
- 指针变量也绑定一个值，与其它变量绑定不一样的值，它绑定一种特殊值，就是内存地址；
- 在绑定的内存地址值下，也是一个变量；

## 什么是引用`Reference`

　　[引用德文][id_02]
> Eine Referenz ist ein Verweis auf ein Objekt. Eine Referenz ist damit ein Aliasname für ein bereits bestehendes Objekt.

直接翻译：引用是关于对象的参照物。因此引用是现有对象的别名。

解读：
- 引用是总是与一个对象关联起来的；
- 从本质上，引用就是相关联的对象，只是表现形式不同而已；

## Rust语言的引用`Reference`与指针`Pointer`

　　在Rust语言里，所有指针都是其自己的类型。且存在很多的指针`pointer`类型。比如，从Rust语言结构上分析，引用`reference`类型是一种指针类型，且最简单的指针类型，从内容上分析，每一个指针类型都包含引用的内存地址。引用`reference`类型的对象是最常见的。下面通过最简单的示意图了解什么是指针概念：

![image](../../hello-borrowing/images/hello_borrowing-05_reference_integer.png)

　　从说明示意图结构上看到，引用`reference`类型的对象`ref_u8`与整数u8类型的对象`instance`是完全一样的，只是其值不同而已。

　　但是Rust语言在处理其值的方法有所不同。这是因为，Rust通常专注于非指针对象的值或者指针对象的引用对象值（即内容中有趣的部分），而不是指针对象的标识（内存地址）。比如下面代码，使用宏println!简单格式打印它们的值都是一样的，只有使用特殊格式，才能打印出引用对象的值，即内存地址。

```rust
let instance = 42_u8;
let ref_u8 = &instance;
println!("{}", instance);
println!("{}", ref_u8);
println!("{:p}", ref_u8);
```

　　Rust语言指针类型可以分为三大类：引用（包括[共享引用和可变引用][id_04]）、[原始引用][id_03]和智能引用`smart pointer`。其中前面两类类是Rust语言本身的，而第三类是属于标准库的。这里重点说明共享引用和原始引用。

　　Ⓘ　共享引用指向其他值所拥有的内存。当创建共享引用值时，引用将防止该值的直接改变。除了下面说明的原始指针之外，Rust语言其他指针都是安全的，并且都具有其生命周期。

　　Ⓘ　原始指针是没有安全性保证的指针。

　　在Rust代码中通常不提倡和不鼓励使用原始指针。Rust语言的原始指针与C语言的指针是等效的。原始指针可以为null，也可以指向垃圾，它们也没有生命周期。

　　为了以后说明问题简单化，我们把上面示意图统一成如下形式：

![image](../../hello-borrowing/images/hello_borrowing-04_reference_integer.png)


## 引用实例解释

![image](../../hello-borrowing/images/hello_borrowing-08_reference_str.png)

　　借助于上面两个引用和一个原始引用的示意图，理解下面相关代码的意义。

　　在函数`main()`里，一共有四段代码，前面两段代码形成了上面示意图内容，后面两段代码是获取它们的地址，以便说明问题。

{{#playpen ../../../../hello-borrowing/bin-hello/examples/use_references_simple.rs editable}}

　　第一段的两行代码是等效的，实际只需要一行代码就可以了。这代码把字符串文字绑定了变量`instance`同时，也形成了原始指针，变量`instance`的地址值指向了原始指针的地址。

　　第二段的两行代码也是等效的，实际只需要一行代码就可以了。这代码把变量`instance`绑定到新变量`copy_instance`，这种绑定方式Rust语言称之为复制（Copy），这里它不会产生新原始指针，而是指向与变量`instance`相同的原始指针地址。

　　从下面程序输出结果，也可以得到验证上面的阐述。第一行和第二行的地址就是原始指针的内存地址，而第三行和第四行的地址是两个引用变量自身的内存地址，注意，它们不是变量的内容值（内存地址）。

```
───────┬──────────────────────────────────────────────────────────────────────────
       │ STDIN
───────┼──────────────────────────────────────────────────────────────────────────
   1   │ instance reference address = 0x101c39b20
   2   │ copy_instance reference address = 0x101c39b20
   3   │ instance address = 0x7fff5dfe81a0
   4   │ copy_instance address = 0x7fff5dfe81c0
───────┴──────────────────────────────────────────────────────────────────────────
```

## 题外话

### 介绍Rust的指针类型列表

| 种类 | 类型 | 名称 | 说明 |
|---|---|---|---|
| 共享引用 | &T | 引用`Reference` | 允许一个或者多个引用来类型T |
| 可变引用 | &mut T | 可变引用`Mutable Reference` | 仅允许单个引用来读和写类型T |
| 智能引用 | Box<T> | Box指针 | 处于堆上类型T的指针类型，该类型只能有单个所有者，它可以读取和写入类型T。 |
| 智能引用 | Rc<T> | 参考计数指针 | 处于堆上类型T的指针类型，该类型可以有多个所有者，它们可以读取类型T。 |
| 智能引用 | Arc<T> | 核参考计数指针 | 与类型Rc<T>一样，但适用于在线程之间安全共享 |
| 原始引用 | *const T | 原始指针`Raw pointer` | 不安全地读取访问类型T |
| 原始引用 | *mut T | 可变原始指针`Mutable raw pointer` | 不安全地读取和写入访问类型T |

![image](../../hello-borrowing/images/rust_container_pointer.png)

### 开发工具：软件篋prettytable-rs

　　使用工具软件篋prettytable-rs可以使得程序输出更加美观。

　　为了使用该工具，需要将下面代码放入文件Cargo.toml的`[dependencies]`段里：

```
prettytable-rs = "0.8.0"
```

　　具体使用实例代码如下：

{{#playpen ../../../../hello-borrowing/bin-hello/examples/use_references.rs editable}}

　　程序输出结果：

```
───────┬─────────────────────────────────────────────────────────────────────────────────────────────────────
       │ STDIN
───────┼─────────────────────────────────────────────────────────────────────────────────────────────────────
   1   │ +---------------------------------+----------------+------------------------------------+
   2   │ | Name                            | Value          | Remark                             |
   3   │ +---------------------------------+----------------+------------------------------------+
   4   │ | instance reference address      | 0x104f58ba0    | is equal to the following line     |
   5   │ +---------------------------------+----------------+------------------------------------+
   6   │ | copy_instance reference address | 0x104f58ba0    |                                    |
   7   │ +---------------------------------+----------------+------------------------------------+
   8   │ | instance address                | 0x7fff5ad3fa50 | is not equal to the following line |
   9   │ +---------------------------------+----------------+------------------------------------+
  10   │ | copy_instance address           | 0x7fff5ad3fa60 |                                    |
  11   │ +---------------------------------+----------------+------------------------------------+
───────┴─────────────────────────────────────────────────────────────────────────────────────────────────────
```

## 参考资料
- [crate chars](https://crates.io/crates/chars)
- [how-can-i-get-an-array-or-a-slice-from-a-raw-pointer](https://stackoverflow.com/questions/27150652/how-can-i-get-an-array-or-a-slice-from-a-raw-pointer)
- [mutation-slice-from-raw-pointer](https://users.rust-lang.org/t/mutation-slice-from-raw-pointer/19912)
- [std slice fn.from_raw_parts](https://doc.rust-lang.org/std/slice/fn.from_raw_parts.html)
- [why-does-printing-a-pointer-print-the-same-thing-as-printing-the-dereferenced-po](https://stackoverflow.com/questions/27852613/why-does-printing-a-pointer-print-the-same-thing-as-printing-the-dereferenced-po)
- [Rust的指针类型](https://shahuwang.com/%E7%BC%96%E7%A8%8B%E8%AF%AD%E8%A8%80/Rust%20Pointers.html) 
- [pointers cheat-sheet](https://www.cs.brandeis.edu/~cs146a/rust/doc-02-21-2015/book/pointers.html#cheat-sheet)
- [Rust learning notes](http://chenju2k6.github.io/blog/2019/05/rustlearn)
- [why-the-machine](https://medium.com/@orbitalK/why-the-machine-b9803a77fa29)


[id_01]:https://en.wikipedia.org/wiki/Pointer_(computer_programming)
[id_02]:https://de.wikipedia.org/wiki/Referenz_(Programmierung)
[id_03]:https://doc.rust-lang.org/guide-pointers.html
[id_04]:https://doc.rust-lang.org/reference/types/pointer.html