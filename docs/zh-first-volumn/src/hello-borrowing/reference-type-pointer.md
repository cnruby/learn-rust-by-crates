# 引用、类型与原始指针解释

## 学习内容
- 了解和学习Rust语言引用`Reference`、类型与原始指针`Pointer`关系

## 篇目

- [引用类型`&str`与原始指针关系图](#引用类型str与原始指针关系图)
- [引用类型`&String`与原始指针关系图](#引用类型String与原始指针关系图)
- [引用、类型与原始指针解释](#引用、类型与原始指针解释)
- [题外话](#题外话)
- [开发工具Cha(rs)](#开发工具Chars)
- [开发工具：软件篋ripgrep](#开发工具：软件篋ripgrep)
- [参考资料](#参考资料)


## 引用类型`&str`与原始指针关系图

![image](../../hello-borrowing/images/hello_borrowing-07-pointers.png)

{{#playpen ../../../../hello-borrowing/bin-hello/examples/use_raw_pointer_str.rs editable}}

### 第一段代码：绑定字符串`String`类型的变量`instance`

```rust,no_run,noplaypen
{{ #include ../../../../hello-borrowing/bin-hello/examples/use_raw_pointer_str.rs:string-type-variable }}
```

　　在这一段代码里，第一个`let`绑定了字符串`String`类型变量`instance`。这样就产生了上面图的变量`instance`及其原始指针。

　　字符串`String`类型由三个部分组成：指向原始指针的指针地址、其长度和容量。该指针地址指向内部缓冲字符串（buffer string），用于存储其数据。

　　在这一段代码里，第二个`let`绑定了`*const u8`类型变量`ref_raw`。其中方法`as_ptr()`功能是将字符串切片转换为原始指针。

### 第二段代码：绑定字符串文字`&str`类型的变量`instance`

```rust,no_run,noplaypen
{{ #include ../../../../hello-borrowing/bin-hello/examples/use_raw_pointer_str.rs:str-type-variable }}
```

　　在这一段代码里，前面四个`let`绑定了字符串文字`＆str`类型变量`ref_str`，它们是完全等效的。其中方法`as_str()`功能是提取包含整个`String`的字符串切片。

　　在这一段代码里，最后的`let`绑定了`*const u8`类型变量`ref_raw_str`。特别需要注意的是，变量`ref_raw_str`也是原始指针的地址。

### 第三段代码：第一次验证原始指针地址

　　这里将验证，第一段代码和第二段代码的原始指针变量`ref_raw`和`ref_raw_str`的地址是相等的。

```rust,no_run,noplaypen
{{ #include ../../../../hello-borrowing/bin-hello/examples/use_raw_pointer_str.rs:str-testing }}
```

### 第四段代码：原始指针内存地址的数组切片

　　在这一段代码里，使用了关键词`unsafe`，然后启动一个包含不安全代码的新代码块。该代码块存在方法from_raw_parts()，它是根据原始指针的引用及其长度返回一个内存地址数组切片。这数组切片的每一项是字符串其中一个字符的内存地址。

```rust,no_run,noplaypen
{{ #include ../../../../hello-borrowing/bin-hello/examples/use_raw_pointer_str.rs:show-raw-address }}
```

### 第五段代码：：第二次验证原始指针地址

　　这里将验证，两个原始指针，第一段代码的变量`ref_raw`和第四段代码数组切片`ref_slice`的第一项的地址是相等的。

```rust,no_run,noplaypen
{{ #include ../../../../hello-borrowing/bin-hello/examples/use_raw_pointer_str.rs:raw-testing }}
```

### 第六段代码：原始指针字符值的数组切片

　　在这一段代码里，使用了关键词`unsafe`，然后启动一个包含不安全代码的新代码块。该代码块存在方法from_raw_parts()，它是根据原始指针及其长度返回一个类型u8数组切片。这类型u8数组切片的每一项是字符串其中的一个字符值。

```rust,no_run,noplaypen
{{ #include ../../../../hello-borrowing/bin-hello/examples/use_raw_pointer_str.rs:show-raw-u8 }}
```

### 程序输出结果

　　该程序输出结果如下，可以比较上面的阐述：

```bash
[bin-hello/examples/use_raw_pointer_str.rs:35] ref_slice = [
    0x00007fa772403730,
    0x00000001034db528,
    0x0000000000000002,
    0x0000000000000000,
    0x00007fff5c750d80,
]
[bin-hello/examples/use_raw_pointer_str.rs:45] u8_slice = [
    72,
    101,
    108,
    108,
    111,
]
───────┬─────────────────────────────────────────────────────────────────────────
       │ STDIN
───────┼─────────────────────────────────────────────────────────────────────────
   1   │ 
   2   │ instance value = Hello
   3   │ instance reference raw address = 0x7fa772403730
   4   │ 
   5   │ ref_str value = Hello
   6   │ ref_str owned address = 0x7fa772403730
   7   │ 
   8   │ 
───────┴─────────────────────────────────────────────────────────────────────────
```

## 引用类型`&String`与原始指针关系图

![image](../../hello-borrowing/images/hello_borrowing-01-pointers.png)

{{#playpen ../../../../hello-borrowing/bin-hello/examples/use_raw_pointer_string.rs editable}}

### 第二段代码：绑定字符串引用`&String`类型的变量`instance`

```rust,no_run,noplaypen
{{ #include ../../../../hello-borrowing/bin-hello/examples/use_raw_pointer_string.rs:string-ref-type-variable }}
```

　　这是唯一一段代码与前面实例代码不同的。在这一段代码里，前面两个`let`绑定了字符串引用`&String`类型的变量`ref_string`，它们是完全等效的。特别需要指出的是，在关键词`let`等式右边类型定义有时候是必要的，尽管这个实例可以省略，但是在上面实例里，就是必须的。

　　在这一段代码里，最后的`let`绑定了`*const u8`类型变量`ref_raw_string`。

### 程序输出结果

　　该程序输出结果如下。比较上面实例结果，可以看到：在这个实例结果只有两个内存地址是一样的，而上面实例有三个内存地址是完全相同的。上面两个结构图就是示意这个结果。

```
[bin-hello/examples/use_raw_pointer_string.rs:32] slice = [
    0x00007fac6ac03770,
    0x0000000101bc4520,
    0x0000000000000002,
    0x0000000000000000,
    0x00007fff5e066ca0,
]
[bin-hello/examples/use_raw_pointer_string.rs:41] slice = [
    72,
    101,
    108,
    108,
    111,
]
───────┬─────────────────────────────────────────────────────────────────────────
       │ STDIN
───────┼─────────────────────────────────────────────────────────────────────────
   1   │ 
   2   │ instance value = Hello
   3   │ instance reference raw address = 0x7fac6ac03770
   4   │ 
   5   │ ref_string value = Hello
   6   │ ref_string owned address = 0x7fff5e066ca0
   7   │ 
   8   │ 
───────┴─────────────────────────────────────────────────────────────────────────
```

## 引用、类型与原始指针解释

　　上面两个实例有什么不同？或者说，引用类型`&String`与引用类型`&str`的区别在哪里？

　　引用类型`&String`是通过类型`String`访问原始指针，而引用类型`&str`是直接访问原始指针。引用类型`&str`的方法是Rust语言借用系统的组成部分。这种方法更安全和更快速。我们把上面两个示意图合并到一起，如下所示：

![image](../../hello-borrowing/images/hello_borrowing-06-pointers.png)

　　从下面示意图可以看到，Rust语言的数据类型可以理解为是一种复杂的“引用”类型。引用类型主要是储存内存地址，而数据类型尽管也储存内存地址，但其重点是储存数据。

![image](../../hello-borrowing/images/hello_borrowing-02_references.png)

### 软件篋Cha(rs)

　　工具Cha(rs)可以显示各种ASCII和unicode字符/代码指针的名称和编码号。

```bash
# install
cargo install chars
# use
chars 'H'
```

### 软件篋ripgrep

　　ripgrep是一款系统终端的搜索工具，类似于ack和grep。

```bash
# install
cargo install ripgrep
# use
ifconfig | rg netmask
```

## 参考资料
- [pointers-in-rust-a-guide](https://words.steveklabnik.com/pointers-in-rust-a-guide)
- [www.cs.brandeis.edu pointers](https://www.cs.brandeis.edu/~cs146a/rust/doc-02-21-2015/book/pointers.html)
- [std string struct.String](https://doc.rust-lang.org/std/string/struct.String.html)
- [Programming Rust (2016)](https://apprize.info/programming/rust/5.html)
- [rust learn](http://chenju2k6.github.io/blog/2019/05/rustlearn)
- [which-problems-does-owning-ref-solve](https://users.rust-lang.org/t/which-problems-does-owning-ref-solve/29245/4)
- [crates chars](https://crates.io/crates/chars)
- [crates ripgrep](https://crates.io/crates/ripgrep)