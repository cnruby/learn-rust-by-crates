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

　　字符串`String`类型由三个部分组成：指向原始指针的指针地址、其长度和容量。该指针地址指向内部缓冲字符串（buffer string），用于存储其数据。

方法`as_ptr()`功能是将字符串切片转换为原始指针。
方法`as_str()`功能是提取包含整个`String`的字符串切片。

{{#playpen ../../../../hello-borrowing/bin-hello/examples/use_raw_pointer_str.rs editable}}

![image](../../hello-borrowing/images/hello_borrowing-07-pointers.png)



## 引用类型`&String`与原始指针关系图

![image](../../hello-borrowing/images/hello_borrowing-01-pointers.png)

{{#playpen ../../../../hello-borrowing/bin-hello/examples/use_raw_pointer_string.rs editable}}


## 引用、类型与原始指针解释

　　引用类型`&String`与引用类型`&str`的区别

![image](../../hello-borrowing/images/hello_borrowing-06-pointers.png)

　　Rust语言的数据类型可以理解为是一种复杂的“引用”类型。

![image](../../hello-borrowing/images/hello_borrowing-02_references.png)

### 软件篋Cha(rs)

cargo install chars
chars 'H'

### 软件篋ripgrep
cargo install ripgrep
ifconfig | rg netmask


## 参考资料
- [std string struct.String](https://doc.rust-lang.org/std/string/struct.String.html)