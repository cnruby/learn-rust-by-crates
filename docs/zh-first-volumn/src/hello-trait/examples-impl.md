# 实现关键词impl与方法代码实现

## 学习内容
- 阐述关键词impl概念
- 理解实现功能关键词impl代码

## 篇目

1. [关键词impl概念](#关键词impl概念)
1. [实现功能关键词impl](#实现功能关键词impl)
1. [实现功能关键词impl与函数代码实现](#实现功能关键词impl与函数代码实现)
1. [参考资料](#参考资料)

## 关键词impl概念

　　关键词impl是为类型实现结构类型或者其它一些类型的不同行为功能。这里仅仅说明了直接针对结构类型本身的不同行为实现方式。

![image](../../images/hello-trait-12-impl.png)

## 实现功能关键词impl

　　Rust语言规定，Ⓓ 始终是公开的，且不可增加修饰关键词pub；Ⓓ 函数和方法默认是私有的，且可增加修饰关键词pub，默认实例代码如下所示。尽管下面代码已实现了函数new()，但是该函数外部还是不可访问的。

```rust
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: String, age: u32) -> Person {
        Person {
            name: name,
            age: age,
        }
    }
}
```

## 实现功能关键词impl与函数代码实现

　　通过下面的代码，可以学习到这些知识：

- 使用关键词impl，实现结构类型StructType的实例化函数new()。
- 使用关键词pub，实现结构类型StructType的函数new()公开性。
- 实例是一种类型的具体对象。
- 借助于公开性函数new()，实现该结构类型的实例化方式。
- 借助于结构类型属性的公开性，实现了实例调用结构类型的属性手法。
- 使用宏方法assert_eq!，验证实例调用结果的正确性。

{{#playpen ../../../../hello-trait/lib-hello/examples/function_instance.rs editable}}

## 参考资料
- [std keyword impl](https://doc.rust-lang.org/std/keyword.impl.html)
- [reference implementations](https://doc.rust-lang.org/reference/items/implementations.html)