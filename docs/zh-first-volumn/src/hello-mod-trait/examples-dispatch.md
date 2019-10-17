# 理解动态与静态调度实现

　　在这一节里，介绍Rust语言调度动态与静态函数的方式。动态与静态调度函数是计算机语言广泛应用的概念和技术。

## 学习内容
- 了解和学习Rust语言静态和动态函数概念
- 理解和掌握静态和动态函数基本实现

## 篇目
- [最常见静态函数实例](#最常见静态函数实例)
- [静态函数的动态调度实例](#静态函数的动态调度实例)
- [调用动态函数的实例](#调用动态函数的实例)
- [题外话](#题外话)
- [原始标识符前缀`r#`](#原始标识符前缀r)
- [数据类型`Vec`](#数据类型Vec)
- [参考资料](#参考资料)

## 最常见静态函数实例

　　Ⓓ 在默认情况下，Rust语言方法都是静态函数。如下面代码的函数`static_dispatch()`。

　　静态调度或者分派（static dispatch）在编译时就知道被调用方是谁，而动态调度只有在运行时，才知道被调用方是谁。显然，在常见的情况下，相比动态调度，在运行程序时，静态调度会更快速，而相比静态调度，在编译程序时，动态调度会更快速。

　　在使用静态分派会更有效，因为总是可以使用静态分派包装器函数来执行动态调度，但反之则不然。由于这个原因，比如标准库尝试在尽可能的情况下进行静态调度。

{{#playpen ../../../../hello-mod-trait/lib-hello/examples/simple_static_dispatch.rs editable}}

## 静态函数的动态调度实例

　　上面程序与下面程序的前面一部分是完全一致的。而下面程序的后面一部分代码动态方式调度静态函数。

　　下面程序的关键词`dyn`就是告诉编译器需要进行动态方式调度。但是，要是关键词`dyn`不在，也能够通过编译，只是有警告提示“不带显式`dyn`的衔接特质对象已弃用”，这告诉我们代码需要增加动态调度关键词`dyn`。

　　这个警告提示提供了一条信息：我们正在涉及到的是衔接特质的一个或者一组对象，之所以需要动态方式，是因为，在Rust语言里衔接特质关键词`trait`可以针对任何类型实现功能，这是一种未知类型行为的随时调度方式。衔接特质的对象如同一个衔接插口，可以随时插到任何一个类型上。

　　在动态调度时，Rust语言需要衔接特质对象的指针。从下面后面的三段代码里，可以理解到这个概念。比如。指针类型`Box`把实例`instance_struct`包装为指针类型；类型Vec的内部项都是实例`instance_struct`的指针类型。

{{#playpen ../../../../hello-mod-trait/lib-hello/examples/simple_dynamic_dispatch.rs editable}}

## 调用动态函数的实例

　　这里将说明基于衔接特质关键性`trait`的静态和动态函数实现，但是这静态函数与之前的也是完全不一样的概念。另外将会看到调用这种动态函数，它们看起来是一些更复杂的静态和动态函数。

　　下面程序第二段代码的两个函数`static_dispatch()`和`dynamic_dispatch()`，它们的目的是解决代码重复的相同问题。但是其手段是不同的。

　　Rust语言没有继承概念，继承编程不再是软件开发的思想。通过关键词`trait`定义函数，借助于关键词impl实现函数及其泛型编程方法，以实现多态式编程方法。静态函数`static_dispatch()`使用了泛型编程方法，关于泛型编程将有另外专题说明。

　　Rust语言也通过衔接特质对象及动态调度编程方法，来实现多态式编程方法。动态函数`dynamic_dispatch()`使用了动态编程方法。特性对象是不限类型的，动态绑定类型是通过实时运行时具体地匹配类型。

　　从代码上看，静态函数`static_dispatch()`和动态函数`dynamic_dispatch()`都是实现相同的功能。

　　下面程序第三段代码里，无论是类型`NormalStruct`，还是类型`TupleStruct`，它们都是以同一方式分别调用函数`static_dispatch()`和函数`dynamic_dispatch()`。

　　不管是静态函数，还是动态函数，它们都是基于衔接特质对象的指针实现，这一点是非常重要的。

　　下面程序是Rust语言非常经典的代码结构。

{{#playpen ../../../../hello-mod-trait/lib-hello/examples/trait_dispatch_concrete.rs editable}}

## 题外话
### 原始标识符前缀`r#`

　　原始标识符也是一种标识符，其前缀是`r#`，之后也可以加上任何严格或保留的关键字，但除关键字`crate, extern, self, super, Self`外。

### 向量数据类型`Vec`

　　向量数据类型`Vec`也是一种数组，其内部是以0开始进行排序的，但是这种数组大小是可调整的，或者说是一种连续的且可增长的数组类型。

## 参考资料
- [reference identifiers](https://doc.rust-lang.org/reference/identifiers.html)
- [static-and-dynamic-dispatch](https://www.cs.brandeis.edu/~cs146a/rust/doc-02-21-2015/book/static-and-dynamic-dispatch.html)
- [book static-and-dynamic-dispatch](https://doc.rust-lang.org/1.0.0-beta/book/static-and-dynamic-dispatch.html)
- [when-does-dynamic-vs-static-dispatch-matter](https://jmarcher.io/when-does-dynamic-vs-static-dispatch-matter/)
- [riptutorial.com static-and-dynamic-dispatch](https://riptutorial.com/rust/example/4656/static-and-dynamic-dispatch)
- [exploring-dynamic-dispatch-in-rust](https://alschwalm.com/blog/static/2017/03/07/exploring-dynamic-dispatch-in-rust/)
- [rust-traits-and-trait-objects](https://joshleeb.com/posts/rust-traits-and-trait-objects/)
- [Static vs Dynamic dispatch](https://gist.github.com/greister/37289c6eb3629d4fefa7dd0acf6de378)
- [a_quick_look_at_trait_objects_in_rust](https://tratt.net/laurie/blog/entries/a_quick_look_at_trait_objects_in_rust.html)
- [b5.impls_and_traits](https://learning-rust.github.io/docs/b5.impls_and_traits.html)
- [std keyword dyn](https://doc.rust-lang.org/std/keyword.dyn.html)
- [Vector of objects belonging to a trait](https://stackoverflow.com/questions/25818082/vector-of-objects-belonging-to-a-trait/25819164)