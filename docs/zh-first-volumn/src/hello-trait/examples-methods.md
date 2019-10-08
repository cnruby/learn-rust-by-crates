# 方法代码实现

## 学习内容
- 阐述Rust语言函数与方法概念区别
- 理解实现函数与方法手段


## 篇目

1. [函数与方法概念](#函数与方法概念)
1. [方法代码实现](#方法代码实现)
1. [参考资料](#参考资料)

## 函数与方法概念

　　在Rust语言里，给予了函数（fcuntion）与方法（method）两个名称不同的概念。

　　引用：[Methods are functions attached to objects][id_01]，直接翻译：方法是附加到对象的函数（行为功能），可以这么理解，方法是附属于类型实例的行为功能。

　　而函数是附属于类型的功能。在前面一节里，可以看到，实现类型的函数代码方法及其调用手法。下面通过代码详细说明它们的区别。

## 实现方法代码

　　这里通过下面的代码，说明如下内容：

- 实现结构类型StructType的实例化函数new()。
- 实现结构类型StructType的获取属性内容方法get_data()。
- 实现结构类型StructType的变更属性内容方法set_data()。
- 实现结构类型StructType的属性data是私有的。
- 借助于公开性函数new()，实现该结构类型的实例化方式。
- 结构类型实例实现了调用结构类型的属性手法。
- 使用宏方法assert_eq!，验证实例调用结果的正确性。

{{#playpen ../../../../hello-trait/lib-hello/examples/method_function.rs editable}}

## 参考资料
- [rust-by-example methods](https://doc.rust-lang.org/rust-by-example/fn/methods.html)

[id_01]:https://doc.rust-lang.org/rust-by-example/fn/methods.html