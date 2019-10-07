# 类型关键词struct

　　通过了解和学习本节Rust语言的基本概念，可以实现本章需要开发的共享软件篋程序代码。

## 学习内容
- 理解和掌握Rust语言结构性的类型关键词struct定义形式
- 理解和掌握Rust语言结构性的类型关键词struct实例化方式
- 理解和掌握Rust语言结构性的类型关键词struct调用手段

## 篇目

1. [类型关键词struct是什么](#类型关键词struct是什么)
1. [C语言形式的类型关键词struct代码](#C语言形式的类型关键词struct代码)
1. [元组形式的类型关键词struct](#元组形式的类型关键词struct)
1. [参考资料](#参考资料)

## 类型关键词struct是什么

　　类型关键词struct提供定义一种结构性的类型方式。这种类型的定义分析如下：

| 名称 | 事物描述 | Rust语言描述 |
|---|---|---|
| 结构性事物 | 类别 | struct |
| 事物名称 | 人 | Person |
| 事物属性 | 姓名，年龄 | name: string, age:u32 |

![image](../../images/hello-trait-11-struct.png)

　　从上图所示，可以理解到，结构类型关键词struct将一组不同的数据类型作为整体在一起分析和处理。

　　同时还可以看到，在Rust语言里，结构类型关键词struct可以将这种结构性事物以两种不同的表达形式进行定义。它们分别称之为：C语言形式和元组形式。图上左边的C语言形式是以哈希结构表达的类型，而图上右边的元组形式是以数组结构表达的类型。

## 类型关键词struct的私有性

　　Rust语言规定，Ⓓ 默认情况下，关键词struct定义的类型及其属性都是私有的，默认实例代码如下所示：

```rust
struct Person {
    name: String,
    age: u32,
}
```

## C语言形式的类型关键词struct代码

　　这里通过下面的代码，说明如下内容：

- 使用关键词mod，关键词struct定义结构类型的方式。
- 使用关键词pub，实现关键词struct定义的类型及其属性公开性。
- 使用结构类型属性的公开性，实现结构类型的实例化方式。
- 结构类型的实例调用结构类型的属性手法。
- 使用宏方法assert_eq!，验证实例调用结果的正确性。

{{#playpen ../../../../hello-trait/lib-hello/examples/pub_field.rs editable}}


## 元组形式的类型关键词struct

　　通过下面的代码，可以学习到这些知识：

- 使用关键词mod，关键词struct定义结构类型的方式。<br/>ⓡ 注意：元组形式的类型关键词struct定义的类型是以分号结束的，而C语言形式的类型关键词struct代码是没有分号的。
- 使用关键词pub，实现关键词struct定义的类型及其属性公开性。
- 使用结构类型属性的公开性，实现结构类型的实例化方式。两种形式的类型不同的。
- 结构类型的实例调用结构类型的属性手法。元组形式的类型是以其属性的顺序号实现调用的，而C语言形式的类型是以其公开性属性实现调用的。
- 使用宏方法assert_eq!，验证实例调用结果的正确性。

{{#playpen ../../../../hello-trait/lib-hello/examples/tuple_struct.rs editable}}


## 参考资料
- [structs from 'rust-by-example '](https://doc.rust-lang.org/stable/rust-by-example/custom_types/structs.html)











