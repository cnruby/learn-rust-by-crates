# 方法代码实现

## 学习内容
- 阐述Rust语言函数与方法概念区别
- 理解实现函数与方法手段

## 篇目

1. [修饰实例关键词mut概念](#修饰实例关键词mut概念)
2. [函数与方法概念](#函数与方法概念)
3. [实现方法代码](#实现方法代码)
4. [结构类型属性的私有性](#结构类型属性的私有性)
5. [参考资料](#参考资料)

## 修饰实例关键词mut概念

```rust, editable
fn main() {
    let instance = 1;
    // instance = 2;
    let mut instance = 1;
    instance = 2;
}
```

## 函数与方法概念

Methods are 功能 attached to objects.
Functions are 功能 attached to types.

## 实现方法代码

　　通过下面的代码，可以学习到这些知识：

- 实现结构类型StructType实例的方法：get_data()和set_data()。
- 借助于公开性方法get_data()，实现获取该结构类型属性的方式。
- 借助于结构类型属性的公开性，实现了实例调用结构类型的属性手法。
- 使用宏方法assert_eq!，验证实例调用结果的正确性。

{{#playpen ../../../../hello-trait/lib-hello/examples/function_method.rs editable}}

## 结构类型属性的私有性

　　结构类型属性的私有性


## 参考资料
- [rust-by-example methods](https://doc.rust-lang.org/rust-by-example/fn/methods.html)
- [how-to-print-well-formatted-tables-to-the-console](https://stackoverflow.com/questions/30379341/how-to-print-well-formatted-tables-to-the-console)